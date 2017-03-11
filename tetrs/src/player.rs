
use ::{Piece, Rot, Point};

/// The player.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Player {
	pub piece: Piece,
	pub rot: Rot,
	pub pt: Point,
}

impl Player {
	/// Creates a new player from its components.
	pub fn new(piece: Piece, rot: Rot, pt: Point) -> Player {
		Player {
			piece: piece,
			rot: rot,
			pt: pt,
		}
	}
	/// Returns the player moved left one step.
	pub fn move_left(self) -> Player {
		Player {
			piece: self.piece,
			rot: self.rot,
			pt: Point {
				x: self.pt.x - 1,
				y: self.pt.y,
			},
		}
	}
	/// Returns the player moved right one step.
	pub fn move_right(self) -> Player {
		Player {
			piece: self.piece,
			rot: self.rot,
			pt: Point {
				x: self.pt.x + 1,
				y: self.pt.y,
			},
		}
	}
	/// Returns the player moved down one step.
	pub fn move_down(self) -> Player {
		Player {
			piece: self.piece,
			rot: self.rot,
			pt: Point {
				x: self.pt.x,
				y: self.pt.y - 1,
			},
		}
	}
	/// Returns the player rotated clockwise.
	pub fn rotate_cw(self) -> Player {
		Player {
			piece: self.piece,
			rot: self.rot.cw(),
			pt: self.pt,
		}
	}
	/// Returns the player rotated counter-clockwise.
	pub fn rotate_ccw(self) -> Player {
		Player {
			piece: self.piece,
			rot: self.rot.ccw(),
			pt: self.pt,
		}
	}
}
