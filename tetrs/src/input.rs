/*!
Game timers.
*/

use ::{Play, State};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Clock {
	pub gravity: i32,
	pub player: i32,
}

#[derive(Default)]
struct InputState {
	move_left: u8,
	move_right: u8,
	soft_drop: u8,
	hard_drop: u8,
	rotate_cw: u8,
	rotate_ccw: u8,
}

pub struct Input {
	speed: Clock,
	timers: Clock,
	state: InputState,
}

impl Input {
	pub fn new(speed: Clock) -> Input {
		Input {
			speed: speed,
			timers: speed,
			state: InputState::default(),
		}
	}

	pub fn move_left_down(&mut self) { self.state.move_left += 1; }
	pub fn move_left_up(&mut self) { self.state.move_left -= 1; }
	pub fn move_right_down(&mut self) { self.state.move_right += 1; }
	pub fn move_right_up(&mut self) { self.state.move_right -= 1; }
	pub fn soft_drop_down(&mut self) { self.state.soft_drop += 1; }
	pub fn soft_drop_up(&mut self) { self.state.soft_drop -= 1; }
	pub fn hard_drop(&mut self) { self.state.hard_drop = 1; }
	pub fn rotate_cw(&mut self) { self.state.rotate_cw = 1; }
	pub fn rotate_ccw(&mut self) { self.state.rotate_ccw = 1; }

	/// Fast forward to the next time new user input will be accepted.
	pub fn ffw(&mut self) -> usize {
		// Advance the timer to the next player input
		self.timers.gravity -= self.timers.player;
		// Fixup gravity timer
		let mut drops = 0;
		while self.timers.gravity < 0 {
			self.timers.gravity += self.speed.gravity;
			drops += 1;
		}
		drops
	}

	pub fn tick(&mut self, state: &mut State) {
		if self.timers.player > 0 {
			self.timers.player -= 1;
		}
		else {
			if self.state.move_left > 0 {

			}
		}
	}
}
