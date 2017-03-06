
use ::{Piece, Rot, Point};

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
	/// Moves the player left.
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
	/// Moves the player right.
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
	/// Drops the player down.
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
	/// Rotates the player clockwise.
	pub fn rotate_cw(self) -> Player {
		Player {
			piece: self.piece,
			rot: self.rot.cw(),
			pt: self.pt,
		}
	}
	/// Rotates the player counter-clockwise.
	pub fn rotate_ccw(self) -> Player {
		Player {
			piece: self.piece,
			rot: self.rot.ccw(),
			pt: self.pt,
		}
	}
}
