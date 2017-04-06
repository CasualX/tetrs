/*!
Simple player bot.
*/

use ::std::{ops, f64};

use ::{Well, Rot, Piece, Player, Point, srs_cw, srs_ccw, test_player, MAX_WIDTH, MAX_HEIGHT};

/// Weights for evaluating well.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Weights {
	/// Factor for the total combined height of the columns.
	pub agg_height_f: f64,
	/// Factor for the max height of the columns.
	pub max_height_f: f64,
	/// Factor for the number of completed lines.
	pub complete_lines_f: f64,
	/// Factor for the number of holes in the field.
	pub holes_f: f64,
	/// Factor for the number of caves in the field.
	pub caves_f: f64,
	/// Factor for the sum of the absolute differences between two adjacent columns.
	pub bumpiness_f: f64,
	/// Factor for the number blocks above a hole.
	pub stacking_f: f64,
}
/// Returns some nice weights.
///
/// Gently appropriated from https://codemyroad.wordpress.com/2013/04/14/tetris-ai-the-near-perfect-player/
impl Default for Weights {
	fn default() -> Weights {
		Weights {
			agg_height_f: -0.510066,
			max_height_f: -0.510066,
			complete_lines_f: 0.760666,
			holes_f: -0.35663,
			caves_f: 0.0,
			bumpiness_f: -0.184483,
			stacking_f: -0.5,
		}
// Weights {
//     agg_height_f: -0.2803344111164008,
//     max_height_f: 0.02526504071606306,
//     complete_lines_f: 0.20605120395222354,
//     holes_f: -0.18751829871729053,
//     caves_f: -0.3557762709568737,
//     bumpiness_f: -0.12041213579170762,
//     stacking_f: -0.06944294190822053
// }
	}
}
impl ::rand::Rand for Weights {
	fn rand<R: ::rand::Rng>(rng: &mut R) -> Weights {
		Weights {
			agg_height_f: rng.gen::<f64>() - 0.5,
			max_height_f: rng.gen::<f64>() - 0.5,
			complete_lines_f: rng.gen::<f64>() - 0.5,
			holes_f: rng.gen::<f64>() - 0.5,
			caves_f: rng.gen::<f64>() - 0.5,
			bumpiness_f: rng.gen::<f64>() - 0.5,
			stacking_f: rng.gen::<f64>() - 0.5,
		}
	}
}
impl Weights {
	/// Evaluates a well and returns a score.
	///
	/// The score is the sum of result of each category multiplied by the appropriated multiplier.
	///
	/// This value only has meaning in comparison to other wells.
	/// A higher value indicates a better scoring well.
	pub fn eval(&self, well: &Well) -> f64 {
		// Quick hack to detect game over
		let lines = well.lines();
		let height = well.height() as usize;
		if lines[height - 1] != 0 || lines[height - 2] != 0 {
			return f64::NEG_INFINITY;
		}

		let (agg_height, max_height, completed_lines, holes, caves, bumpiness, stacks) = Self::crunch(well);
		return
			self.agg_height_f * agg_height as f64 +
			self.max_height_f * max_height as f64 +
			self.complete_lines_f * completed_lines as f64 +
			self.holes_f * holes as f64 +
			self.caves_f * caves as f64 +
			self.bumpiness_f * bumpiness as f64 +
			self.stacking_f * stacks as f64;
	}
	fn crunch(well: &Well) -> (i32, i32, i32, i32, i32, i32, i32) {
		let width = well.width() as usize;
		let mut heights = [0i32; MAX_WIDTH];
		let mut holes = [0i32; MAX_WIDTH];
		let mut stacks = [0i32; MAX_WIDTH];
		let _ = heights[..width];
		let _ = holes[..width];
		let _ = stacks[..width];
		let mut lines = 0;
		let line_mask = well.line_mask();

		let mut height = 0;
		for &line in well.lines() {
			// Skip cleared lines
			if line == line_mask {
				lines += 1;
			}
			else {
				height += 1;
				let mut line = line;
				for col in 0..width {
					if line & 1 != 0 {
						// Sum the holes for this column
						holes[col] += height - heights[col] - 1;
						// Save the height for this column
						heights[col] = height;
						// Save the stacks for this column
						stacks[col] += (holes[col] != 0) as i32;
					}
					line >>= 1;
				}
			}
		}

		let holes_sum = well.count_holes();
		let height_sum = heights[..width].iter().sum();
		let heights_max = heights[..width].iter().max().cloned().unwrap();
		let caves_sum = holes[..width].iter().fold(0, ops::Add::add) - holes_sum;
		let stacks_sum = stacks[..width].iter().sum();
		let bumpiness = heights[..width].windows(2).map(|window| (window[0] - window[1]).abs()).sum();

		(height_sum, heights_max, lines, holes_sum, caves_sum, bumpiness, stacks_sum)
	}
}

/// Player move.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Play {
	Idle,
	MoveLeft,
	MoveRight,
	RotateCW,
	RotateCCW,
	SoftDrop,
	HardDrop,
}

/// Player AI.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayI {
	pub score: f64,
	pub play: Vec<Play>,
	pub player: Option<Player>,
}

impl PlayI {
	/// Calculate the best move with the given weights.
	pub fn play(weights: &Weights, well: &Well, player: Player) -> PlayI {
		// Keep track of which states we've visited
		// TODO! Use a bit array instead, reduces allocation by a factor of 8
		const STRIDE: usize = (MAX_WIDTH + 3) * 4;
		const SIZE: usize = STRIDE * (MAX_HEIGHT + 4);
		let mut visited = [false; SIZE];
		let mut visit = |next: Player| -> bool {
			let i = (next.pt.y as i32 * STRIDE as i32 + (next.pt.x as i32 + 3) * 4 + next.rot as u8 as i32) as usize;
			if !visited[i] {
				visited[i] = true;
				false
			}
			else {
				true
			}
		};
		// Depth-first traversal through the possible game states
		let mut path = Vec::new();
		path.push((Play::Idle, player));
		// Accumulate for the best possible game state
		let mut best = PlayI {
			score: f64::NEG_INFINITY,
			play: Vec::new(),
			player: None,
		};
		// While we have unexplored game states
		while let Some(&(play, player)) = path.last() {
			match play {
				Play::Idle => {
					path.last_mut().unwrap().0 = Play::SoftDrop;
					let next = player.move_down();
					if !visit(next) {
						if !test_player(well, next) {
							path.push((Play::Idle, next));
						}
						else {
							let mut well = *well;
							etch_player(&mut well, player);
							let score = weights.eval(&well);
							if score > best.score {
								best.score = score;
								best.play.clear();
								best.play.extend(path.iter().map(|&(play, _)| play));
								best.player = Some(player);
							}
						}
					}
				},
				Play::SoftDrop => {
					path.last_mut().unwrap().0 = Play::MoveLeft;
					let next = player.move_left();
					if !visit(next) && !test_player(well, next) {
						path.push((Play::Idle, next));
					}
				},
				Play::MoveLeft => {
					path.last_mut().unwrap().0 = Play::MoveRight;
					let next = player.move_right();
					if !visit(next) && !test_player(well, next) {
						path.push((Play::Idle, next));
					}
				},
				Play::MoveRight => {
					path.last_mut().unwrap().0 = Play::RotateCW;
					let next = srs_cw(well, player);
					if !visit(next) {
						path.push((Play::Idle, next));
					}
				},
				Play::RotateCW => {
					path.last_mut().unwrap().0 = Play::RotateCCW;
					let next = srs_ccw(well, player);
					if !visit(next) {
						path.push((Play::Idle, next));
					}
				},
				Play::RotateCCW => {
					// Exhausted all possible moves, back one up and try again
					path.pop();
				},
				_ => unreachable!(),
			}
		}
		best
	}
	/// Brute force the worst piece for the given well and weights.
	pub fn worst_piece(weights: &Weights, well: &Well) -> Piece {
		let pieces = [Piece::S, Piece::Z, Piece::O, Piece::I, Piece::L, Piece::J, Piece::T];
		pieces[..].iter().fold((pieces[0], f64::INFINITY), |(bad_piece, bad_score), &piece| {
			let score = Self::piece(weights, well, piece);
			if score < bad_score {
				(piece, score)
			}
			else {
				(bad_piece, bad_score)
			}
		}).0
	}
	/// Brute force the best piece for the given well and weights.
	pub fn best_piece(weights: &Weights, well: &Well) -> Piece {
		let pieces = [Piece::T, Piece::J, Piece::L, Piece::I, Piece::O, Piece::Z, Piece::S];
		pieces[..].iter().fold((pieces[0], f64::NEG_INFINITY), |(good_piece, good_score), &piece| {
			let score = Self::piece(weights, well, piece);
			if score > good_score {
				(piece, score)
			}
			else {
				(good_piece, good_score)
			}
		}).0
	}
	fn piece(weights: &Weights, well: &Well, piece: Piece) -> f64 {
		// Recursive floodfill to find all the playable states

		// The number of states in a single row:
		// `MAX_WIDTH` plus `3` (for overlap with the well) times `4` (the number of rotations)
		const STRIDE: usize = (MAX_WIDTH + 3) * 4;
		// The number of rows starting all the way up to the top
		const SIZE: usize = STRIDE * (MAX_HEIGHT + 4);
		// Mark every place with a visited flag to know to not recurse in here
		type Visited = [bool; SIZE];
		let mut visited = [false; SIZE];

		// Recursively visit all states
		fn rec(visited: &mut Visited, weights: &Weights, well: &Well, player: Player) -> f64 {
			// Check if the current position has been visited
			let i = (player.pt.y as i32 * STRIDE as i32 + (player.pt.x as i32 + 3) * 4 + player.rot as u8 as i32) as usize;
			// println!("player:{:?} STRIDE:{}", player, STRIDE);
			if visited[i] {
				return f64::NEG_INFINITY;
			}
			visited[i] = true;
			// Test if this is a valid move
			// FIXME! Does not evaluate wall-kicks!
			if test_player(well, player) {
				return f64::NEG_INFINITY;
			}
			// Try all possible moves from this location
			let cw = rec(visited, weights, well, player.rotate_cw());
			let ccw = rec(visited, weights, well, player.rotate_ccw());
			let left = rec(visited, weights, well, player.move_left());
			let right = rec(visited, weights, well, player.move_right());
			// Finally try moving one down, and eval well
			let player_down = if test_player(well, player.move_down()) {
				let mut well = *well;
				etch_player(&mut well, player);
				weights.eval(&well)
			}
			else {
				rec(visited, weights, well, player.move_down())
			};
			// Brute force for the highest valued placement
			cw.max(ccw).max(left).max(right).max(player_down)
		}

		let start = Player::new(piece, Rot::Zero, Point::new(well.width() / 2 - 2, well.height() + 3));
		rec(&mut visited, weights, well, start)
	}
}

fn etch_player(well: &mut Well, player: Player) {
	let sprite = player.sprite();
	well.etch(sprite, player.pt)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn tdd() {
		let well = Well::from_data(10, &[
			0b0000000000,
			0b0000110000,
			0b0111111001,
			0b0110111111,
			0b1111111111,
			0b1110111111,
			0b1111111111,
		]);
		let (heights_sum, height_max, lines, holes_sum, caves_sum, bumpiness, stacks) = Weights::crunch(&well);
		assert_eq!(28, heights_sum);
		assert_eq!(4, height_max);
		assert_eq!(2, lines);
		assert_eq!(2, holes_sum);
		assert_eq!(0, caves_sum);
		assert_eq!(6, bumpiness);
		assert_eq!(1, stacks);
	}

	#[test]
	fn play() {
		let well = Well::from_data(10, &[
			0b0000000000,
			0b0000000000,
			0b0000000000,
			0b0000000000,
			0b1100110000,
			0b1100111111,
		]);
		let bot = PlayI::play(&Weights::default(), &well, Player::new(Piece::O, Rot::Zero, Point::new(4, 6)));
		use Play::*;
		println!("{:#?}", bot);
		assert_eq!(&[SoftDrop, SoftDrop, MoveLeft, MoveLeft, MoveLeft, SoftDrop, SoftDrop, SoftDrop], &*bot.play);
	}
}
