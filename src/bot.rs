/*!
Simple player bot.
*/

use ::std::f64;

use ::{Well, Rot, Piece, Player, Point, State, MAX_WIDTH};

pub struct PlayerBot {
	heights: f64,
	lines: f64,
	holes: f64,
	bumpiness: f64,
	stacks: f64,
	walltouch: f64,
}

impl PlayerBot {
	pub fn new() -> PlayerBot {
		// Weights from https://codemyroad.wordpress.com/2013/04/14/tetris-ai-the-near-perfect-player/
		PlayerBot {
			heights: -0.510066,
			lines: 0.760666,
			holes: -0.35663,
			bumpiness: -0.184483,
			stacks: -0.5,
			walltouch: 0.0,

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
	pub fn play(&self, state: &mut State) {
		let player = *state.player().unwrap();
		let (x, mut rot) = self.ask(state.well(), player.piece);
		while rot != player.rot {
			assert!(state.rotate_cw());
			rot = rot.ccw();
		}
		if x < player.pt.x {
			for _ in 0..player.pt.x - x {
				assert!(state.move_left());
			}
		}
		else if x > player.pt.x {
			for _ in 0..x - player.pt.x {
				assert!(state.move_right());
			}
		}
		assert!(state.hard_drop());
	}
	pub fn ask(&self, well: &Well, piece: Piece) -> (i32, Rot) {
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
				let walltouch = well.test(&Player::new(piece, rot, Point::new(x - 1, well.height())))
					|| well.test(&Player::new(piece, rot, Point::new(x + 1, well.height())));
				// Drop the piece down
				while !well.test(&player) {
					player.pt.y -= 1;
				}
				player.pt.y += 1;
				// Evaluate the well
				let mut well = well.clone();
				well.etch(&player);
				let mut score = self.eval(&well);
				if walltouch {
					score += self.walltouch;
				}
				// Keep the best scoring move
				if score > best_score {
					best_x = x;
					best_rot = rot;
					best_score = score;
				}
			}
		}
		// println!("x:{} rot:{:?} score:{}", best_x, best_rot, best_score);
		(best_x, best_rot)
	}

	fn eval(&self, well: &Well) -> f64 {
		let (heights, lines, holes, bumpiness, stacks) = Self::crunch(well);
		return
			self.heights * heights as f64 +
			self.lines * lines as f64 +
			self.holes * holes as f64 +
			self.bumpiness * bumpiness as f64 +
			self.stacks * stacks as f64;
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
