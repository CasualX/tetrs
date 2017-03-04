/*!
Playing field.
*/

use ::std::{fmt, ops};
use ::std::str::{FromStr};

use ::{Player, Point};

pub const MAX_HEIGHT: usize = 22;
pub const MAX_WIDTH: usize = 16;

pub type Line = u16;

#[derive(Clone, Debug)]
pub struct Well {
	width: i16,
	height: i16,
	field: [Line; MAX_HEIGHT],
}

impl Well {
	/// Creates a new well with the given dimensions.
	///
	/// # Panics
	///
	/// The width must be ∈ [4, 16] and the height must be ∈ [4, 22].
	pub fn new(width: i32, height: i32) -> Well {
		assert!(width >= 4 && width <= MAX_WIDTH as i32, "width must be ∈ [4, {}]", MAX_WIDTH);
		assert!(height >= 4 && height <= MAX_HEIGHT as i32, "height must be ∈ [4, {}]", MAX_HEIGHT);
		Well {
			width: width as i16,
			height: height as i16,
			field: [0; MAX_HEIGHT],
		}
	}
	pub fn from_data(width: i32, lines: &[Line]) -> Well {
		let mut well = Well::new(width, lines.len() as i32);
		for row in 0..lines.len() {
			well.field[row] = lines[row];
		}
		well
	}
	/// Returns the width of the well.
	pub fn width(&self) -> i32 {
		self.width as i32
	}
	/// Returns the height of the well.
	pub fn height(&self) -> i32 {
		self.height as i32
	}
	/// Returns the field as lines.
	pub fn lines(&self) -> &[Line] {
		&self.field[..self.height as usize]
	}
	pub fn lines_mut(&mut self) -> &mut [Line] {
		&mut self.field[..self.height as usize]
	}
	/// Hit tests the player against the field.
	///
	/// Returns `true` if the player is out of bounds left, right or below the well or if the piece overlaps with an occupied cell; `false` otheriwse.
	pub fn test(&self, player: &Player) -> bool {
		// Early reject out of bounds
		if player.pt.x < (0 - 4) || player.pt.x >= self.width() || player.pt.y < 0 {
			return true;
		}
		if player.pt.y >= self.height() + 4 {
			return false;
		}

		// Get the unperturbed mesh
		let mesh = player.piece.mesh().data[player.rot as u8 as usize];

		// For clipping left/right walls
		let line_mask = if player.pt.x < 0 { self.line_mask() << (-player.pt.x) as usize } else { self.line_mask() >> player.pt.x as usize };

		for y in 0..4 {
			// Check if part is sticking out of a wall
			if (mesh[y as usize] as Line) & !line_mask != 0 {
				return true;
			}
			let row = player.pt.y - y;
			// If this row is below the floor
			if row < 0 {
				if mesh[y as usize] != 0 {
					return true;
				}
			}
			// If this row is below the ceiling
			else if row < self.height() {
				// Render the mesh for this line
				let cg_line = if player.pt.x < 0 {
					(mesh[y as usize] as Line) >> (-player.pt.x) as usize
				}
				else {
					(mesh[y as usize] as Line) << player.pt.x as usize
				};
				if cg_line & self.field[row as usize] != 0 {
					return true;
				}
			}
		}
		return false;
	}
	/// Tests if any block is set on this line.
	pub fn test_line(&self, row: i32) -> bool {
		self.field[row as usize] != 0
	}
	/// Etch the player into the field.
	pub fn etch(&mut self, player: &Player) {
		// Grab the mesh for this rotation
		let mesh = player.piece.mesh().data[player.rot as u8 as usize];
		// Etch the 4x4 mask into the field
		for y in 0..4 {
			// Clip the affected row to the field
			let row = player.pt.y - y;
			if row >= 0 && row < self.height() {
				// Render the mesh for this line
				let cg_line = if player.pt.x < 0 {
					(mesh[y as usize] as Line) >> (-player.pt.x) as usize
				}
				else {
					(mesh[y as usize] as Line) << player.pt.x as usize
				};
				self.field[row as usize] |= cg_line;
			}
		}
	}
	/// Gets a line with all columns set.
	pub fn line_mask(&self) -> Line {
		(1 << self.width() as usize) - 1
	}
	/// Gets a line.
	pub fn line(&self, row: i32) -> Line {
		self.field[row as usize]
	}
	/// Sets a line.
	///
	/// Returns the erased line.
	pub fn set_line(&mut self, row: i32, line: Line) -> Line {
		let old = self.field[row as usize];
		self.field[row as usize] = line;
		old
	}
	/// Removes a line.
	///
	/// Returns the removed line.
	pub fn remove_line(&mut self, row: i32) -> Line {
		let line = self.field[row as usize];
		for i in row as usize..MAX_HEIGHT - 1 {
			self.field[i] = self.field[i + 1];
		}
		line
	}
	/// Inserts a line.
	///
	/// Returns the top line that got bumped out.
	pub fn insert_line(&mut self, row: i32, line: Line) -> Line {
		let old = self.field[self.height() as usize - 1];
		for i in (row as usize..self.height() as usize - 1).rev() {
			self.field[i + 1] = self.field[i];
		}
		self.field[row as usize] = line;
		old
	}
	/// Describes the field.
	pub fn describe<F>(&self, mut f: F) where F: FnMut(Point) {
		let height = self.height();
		let width = self.width();
		let _ = self.field[..height as usize];
		for row in 0..height {
			let mut line = self.field[row as usize];
			for col in 0..width {
				if line & 1 != 0 {
					f(Point::new(col, row));
				}
				line >>= 1;
			}
		}
	}
}

/// Set all the blocks in lhs that aren't set in rhs.
impl ops::Sub for Well {
	type Output = Well;
	fn sub(self, rhs: Well) -> Well {
		assert_eq!(self.width(), rhs.width());
		assert_eq!(self.height(), rhs.height());
		let mut result = Well::new(self.width(), self.height());
		for (result, (&lhs, &rhs)) in Iterator::zip(result.lines_mut().iter_mut(), Iterator::zip(self.lines().iter(), rhs.lines().iter())) {
			*result = lhs & !rhs;
		}
		result
	}
}

pub enum ParseWellError {
	/// The string is empty.
	Empty,
	/// The well has no walls.
	BadWalls,
	/// The well has inconsistent width.
	InWidth,
	/// The well is too wide.
	OutWidth,
	/// The well is too high.
	OutHeight,
}
impl FromStr for Well {
	type Err = ParseWellError;
	fn from_str(s: &str) -> Result<Well, ParseWellError> {
		let mut width = None;
		let mut height = 0;
		let mut field = [0; MAX_HEIGHT];

		for line in s.lines() {
			let line = line.trim_right();
			if line.len() < 3 {
				return Err(ParseWellError::BadWalls);
			}
			let bline = line.as_bytes();
			if bline[0] != b'|' || bline[bline.len() - 1] != b'|' {
				return Err(ParseWellError::BadWalls);
			}
			let mut w = 0;
			let mut row = 0;
			let line = &line[1..line.len() - 1];
			for c in line.chars() {
				let bit = if c == ' ' { 0 } else { 1 };
				row |= bit << w;
				w += 1;
				if w >= MAX_WIDTH {
					return Err(ParseWellError::OutWidth);
				}
			}

			if let Some(prev_width) = width {
				if prev_width != w {
					return Err(ParseWellError::InWidth);
				}
			}
			else {
				width = Some(w);
			}

			field[height] = row;

			height += 1;
			if height >= MAX_HEIGHT {
				return Err(ParseWellError::OutHeight);
			}
		}

		if let Some(width) = width {
			Ok(Well {
				width: width as i16,
				height: height as i16,
				field: field,
			})
		}
		else {
			return Err(ParseWellError::Empty);
		}
	}
}

impl fmt::Display for Well {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut bg = " ";
		for &row in self.field[0..self.height() as usize].iter().rev() {
			f.write_str("|")?;
			let mut mask = 0x1;
			for _ in 0..self.width() {
				let graphic = if (row & mask) != 0 { "□" } else { bg };
				f.write_str(graphic)?;
				mask <<= 1;
			}
			f.write_str("|\n")?;
			if bg == " " {
				bg = "_";
			}
			else if bg == "_" {
				bg = ".";
			}
		}
		f.write_str("+")?;
		for _ in 0..self.width() {
			f.write_str("-")?;
		}
		f.write_str("+")
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	use ::{Piece, Rot, Point};

	fn well() -> Well {
		let mut well = Well::new(10, 4);

		let p1 = Player::new(Piece::L, Rot::Zero, Point::new(2, 1));
		let p2 = Player::new(Piece::O, Rot::Zero, Point::new(-1, 2));
		let p3 = Player::new(Piece::I, Rot::One, Point::new(7, 3));

		well.etch(&p1);
		well.etch(&p2);
		well.etch(&p3);
		println!("\n{}", well);

		well
	}

	#[test]
	fn etch() {
		let well = well();
		assert_eq!("|         □|\n\
		            |         □|\n\
		            |□□   □   □|\n\
		            |□□ □□□   □|\n\
		            +----------+", format!("{}", well));
	}

	#[test]
	fn hit_test() {
		let well = well();
		// Within the field bounds
		assert!(!well.test(&Player::new(Piece::S, Rot::Zero, Point::new(-1, 3))));
		assert!(!well.test(&Player::new(Piece::J, Rot::Three, Point::new(5, 2))));
		// Clip left wall
		assert!(well.test(&Player::new(Piece::S, Rot::Zero, Point::new(-2, 3))));
		// Clip with existing pieces
		assert!(well.test(&Player::new(Piece::I, Rot::Two, Point::new(2, 3))));
		// Clip right wall
		assert!(well.test(&Player::new(Piece::O, Rot::One, Point::new(9, 1))));
		// Clip the bottom
		assert!(well.test(&Player::new(Piece::J, Rot::Three, Point::new(5, 1))));
	}

	#[test]
	fn remove_line() {
		let mut well = well();

		let removed1 = well.remove_line(3);
		assert_eq!(0b1000000000, removed1);
		assert_eq!("|          |\n\
		            |         □|\n\
		            |□□   □   □|\n\
		            |□□ □□□   □|\n\
		            +----------+", format!("{}", well));

		let removed2 = well.remove_line(0);
		assert_eq!(0b1000111011, removed2);
		assert_eq!("|          |\n\
		            |          |\n\
		            |         □|\n\
		            |□□   □   □|\n\
		            +----------+", format!("{}", well));
	}
}
