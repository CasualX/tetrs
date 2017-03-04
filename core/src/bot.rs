/*!
Simple player bot.
*/

use ::std::f64;

use ::{Well, Rot, Piece, Player, Point, State, MAX_WIDTH};

/// Weights for evaluating well.
pub struct PlayW {
	/// Factor for the total combined height of all the columns.
	pub agg_height_f: f64,
	/// Factor for the number of completed lines.
	pub complete_lines_f: f64,
	/// Factor for the number of holes in the field.
	pub holes_f: f64,
	/// Factor for the sum of the absolute differences between two adjacent columns.
	pub bumpiness_f: f64,
	/// Factor for the number blocks above a hole.
	pub stacking_f: f64,
}

impl PlayW {
	/// Returns some nice weights.
	///
	/// Gently appropriated from https://codemyroad.wordpress.com/2013/04/14/tetris-ai-the-near-perfect-player/
	pub fn new() -> PlayW {
		PlayW {
			agg_height_f: -0.510066,
			complete_lines_f: 0.760666,
			holes_f: -0.35663,
			bumpiness_f: -0.184483,
			stacking_f: -0.5,

			// heights: -0.03,
			// lines: 8.0,
			// holes: -7.5,
			// bumpiness: -3.5,
			// walltouch: 6.52,

			// heights: -3.78,
			// lines: 1.6,
			// holes: -2.31,
			// bumpiness: -0.184483,
			// walltouch: 6.52,
		}
	}
	/// Evaluates a well.
	///
	/// This value only has meaning in comparison to other wells.
	/// A higher value indicates a better scoring well.
	pub fn eval(&self, well: &Well) -> f64 {
		let (agg_height, completed_lines, holes, bumpiness, stacks) = Self::crunch(well);
		return
			self.agg_height_f * agg_height as f64 +
			self.complete_lines_f * completed_lines as f64 +
			self.holes_f * holes as f64 +
			self.bumpiness_f * bumpiness as f64 +
			self.stacking_f * stacks as f64;
	}
	fn crunch(well: &Well) -> (i32, i32, i32, i32, i32) {
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

		let height_sum = heights[..width].iter().sum();
		let holes_sum = holes[..width].iter().sum();
		let stacks_sum = stacks[..width].iter().sum();
		let bumpiness = heights[..width].windows(2).map(|window| (window[0] - window[1]).abs()).sum();

		(height_sum, lines, holes_sum, bumpiness, stacks_sum)
	}
}

/// Player move.
pub enum PlayM {
	Idle,
	MoveLeft,
	MoveRight,
	RotateCW,
	RotateCCW,
	SoftDrop,
	HardDrop,
}

/// Player AI.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlayI {
	x: i32,
	rot: Rot,
	score: f64,
}

impl PlayI {
	/// Brute force the best move with the given well and weights.
	pub fn best(weights: &PlayW, well: &Well, piece: Piece) -> PlayI {
		// Brute force a solution...
		let mut best_x = 0;
		let mut best_rot = Rot::Zero;
		let mut best_score = f64::NEG_INFINITY;
		for rot in 0..3 {
			for x in -3..well.width() {
				let rot = Rot::from(rot);
				let mut player = Player::new(piece, rot, Point::new(x, well.height()));
				// Early reject against the walls
				if well.test(&player) {
					continue;
				}
				// Drop the piece down
				while !well.test(&player) {
					player.pt.y -= 1;
				}
				player.pt.y += 1;
				// Evaluate the well
				let mut well = well.clone();
				well.etch(&player);
				let score = weights.eval(&well);
				// Keep the best scoring move
				if score > best_score {
					best_x = x;
					best_rot = rot;
					best_score = score;
				}
			}
		}
		PlayI {
			x: best_x,
			rot: best_rot,
			score: best_score,
		}
	}
	/// Brute force the worst piece for the given well and weights.
	pub fn worst(weights: &PlayW, well: &Well) -> Piece {
		let pieces = [Piece::S, Piece::Z, Piece::O, Piece::I, Piece::L, Piece::J, Piece::T];
		pieces[..].iter().fold((Piece::S, f64::INFINITY), |(bad_piece, bad_score), &piece| {
			let PlayI { score, .. } = Self::best(weights, well, piece);
			if score < bad_score {
				(piece, score)
			}
			else {
				(bad_piece, bad_score)
			}
		}).0
	}
	pub fn play(&self, state: &State) -> PlayM {
		match state.player() {
			Some(&player) => {
				if self.rot != player.rot {
					PlayM::RotateCW
				}
				else if self.x < player.pt.x {
					PlayM::MoveLeft
				}
				else if self.x > player.pt.x {
					PlayM::MoveRight
				}
				else {
					PlayM::HardDrop
				}
			},
			None => PlayM::Idle,
		}
	}
}

#[test]
fn tdd() {
	let well = Well::from_data(10, &[
		0b1111111111,
		0b1111110111,
		0b1111111111,
		0b1111110110,
		0b1001111110,
		0b0000110000,
	]);
	let (heights_sum, lines, holes_sum, bumpiness, stacks) = PlayerBot::crunch(&well);
	assert_eq!(28, heights_sum);
	assert_eq!(2, lines);
	assert_eq!(2, holes_sum);
	assert_eq!(6, bumpiness);
	assert_eq!(1, stacks);
}
