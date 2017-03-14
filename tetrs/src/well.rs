/*!
Playing field.
*/

use ::std::{fmt};
use ::std::str::{FromStr};

use ::{Player, Piece};

/// Maximum well height.
///
/// If this is changed, don't forget to update the documentation for `Well::new`.
///
/// Note that the absolute limit is about `123` (max value for `i8` - `4` for padding).
pub const MAX_HEIGHT: usize = 22;
/// Maxium well width.
///
/// If this is changed, don't forget to update the documentation for `Well::new`.
///
/// Note that the `Line` type should be updated to be able to hold this number of bits (as a bit mask).
pub const MAX_WIDTH: usize = 16;

/// A line is represented by a bit mask in 'reversed' order.
///
/// The block with position `x` is represented by the bit `1 << x` (this is mirrored from its binary form when printed).
///
/// TODO! This mirroring causes a lot of conceptual problems, consider changing it.
pub type Line = u16;

/// Playing field.
///
/// Represents the tetris playing field efficiently using bit masks without memory allocations.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Well {
	width: i8,
	height: i8,
	_pad: i16,
	field: [Line; MAX_HEIGHT],
}

const MINOS_STR: &'static str = "□";

impl Well {
	/// Creates an empty well with the given dimensions.
	///
	/// # Panics
	///
	/// The width must be ∈ [4, 16] and the height must be ∈ [4, 22].
	pub fn new(width: i8, height: i8) -> Well {
		assert!(width >= 4 && width <= MAX_WIDTH as i8, "width must be ∈ [4, {}]", MAX_WIDTH);
		assert!(height >= 4 && height <= MAX_HEIGHT as i8, "height must be ∈ [4, {}]", MAX_HEIGHT);
		Well {
			width: width,
			height: height,
			_pad: 0,
			field: [0; MAX_HEIGHT],
		}
	}
	/// Creates a new well with the given data.
	///
	/// Note that the input lines are in 'visual' order, they will be vertically flipped and horizontally mirrored for internal storage.
	///
	/// # Panics
	///
	/// No minos may be found outside the well's width.
	pub fn from_data(width: i8, lines: &[Line]) -> Well {
		let mut well = Well::new(width, lines.len() as i8);
		for (lhs, &rhs) in Iterator::zip(well.field[..lines.len()].iter_mut(), lines.iter().rev()) {
			let mut rhs = rhs;
			let mut line = 0;
			for _ in 0..width {
				line = (line << 1) | (rhs & 1);
				rhs >>= 1;
			}
			assert_eq!(0, rhs, "found minos outside the well's width");
			*lhs = line;
		}
		well
	}
	/// Returns the width of the well.
	pub fn width(&self) -> i8 {
		self.width
	}
	/// Returns the height of the well.
	pub fn height(&self) -> i8 {
		self.height
	}
	/// Returns the field as lines.
	///
	/// Note that the bottom row sits at index 0 going up the field as the index increases.
	pub fn lines(&self) -> &[Line] {
		&self.field[..self.height as usize]
	}
	/// Hit tests the player against the field.
	///
	/// Returns `true` if the player is out of bounds left, right or below the well or if the piece overlaps with an occupied cell; `false` otheriwse.
	pub fn test_player(&self, player: Player) -> bool {
		// Early reject out of bounds
		if player.pt.x < (0 - 4) || player.pt.x >= self.width || player.pt.y < 0 {
			return true;
		}
		if player.pt.y >= self.height + 4 {
			return false;
		}

		// Get the unperturbed mesh
		let mesh = player.piece.mesh().data[player.rot as u8 as usize];

		// For clipping left/right walls
		let line_mask = if player.pt.x < 0 {
			self.line_mask() << (-player.pt.x) as usize
		}
		else {
			self.line_mask() >> player.pt.x as usize
		};

		// The compiler actually unrolls and splits this loop, pretty slick :)
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
			else if row < self.height {
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
	/// Hit test the player with wall kicks.
	///
	/// If a valid wall kick is found, the input player is modified with the new position and `true` is returned, `false` otherwise.
	///
	/// The actual wall kicks allowed don't follow any advanced rules, it just offsets the position and stops when a valid position is found.
	///
	/// The positions tested are `x - 1`, `x + 1`, `x - 2` and `x + 2` (the latter two are only for the I piece).
	///
	/// TODO! Figure out the flexible SRS wall kick system.
	pub fn test_wall_kick(&self, player: &mut Player) -> bool {
		player.pt.x -= 1;
		if !self.test_player(*player) {
			return false;
		}
		player.pt.x += 2;
		if !self.test_player(*player) {
			return false;
		}
		if player.piece == Piece::I {
			player.pt.x -= 3;
			if !self.test_player(*player) {
				return false;
			}
			player.pt.x += 4;
			if !self.test_player(*player) {
				return false;
			}
			player.pt.x -= 2;
		}
		else {
			player.pt.x -= 1;
		}
		return true;
	}
	/// Traces a player down and returns where it will come to rest.
	pub fn trace_down(&self, mut player: Player) -> Player {
		loop {
			let next = player.move_down();
			if self.test_player(next) {
				return player;
			}
			player = next;
		}
	}
	/// Etch the player into the field.
	pub fn etch(&mut self, player: Player) {
		// Grab the mesh for this rotation
		let mesh = player.piece.mesh().data[player.rot as u8 as usize];
		// Etch the 4x4 mask into the field
		for y in 0..4 {
			// Clip the affected row to the field
			let row = player.pt.y - y;
			if row >= 0 && row < self.height {
				// Render the mesh for this line
				let line_mask = if player.pt.x < 0 {
					(mesh[y as usize] as Line) >> (-player.pt.x) as usize
				}
				else {
					(mesh[y as usize] as Line) << player.pt.x as usize
				};
				self.field[row as usize] |= line_mask;
			}
		}
	}
	/// Gets a line with all columns set.
	pub fn line_mask(&self) -> Line {
		(1 << self.width() as usize) - 1
	}
	/// Gets a line.
	pub fn line(&self, row: i8) -> Line {
		self.field[row as usize]
	}
	/// Sets a line.
	///
	/// Returns the erased line.
	pub fn set_line(&mut self, row: i8, line: Line) -> Line {
		let old = self.field[row as usize];
		self.field[row as usize] = line;
		old
	}
	/// Removes a line.
	///
	/// Returns the removed line.
	///
	/// The lines above the removed line are shifted down and an empty line is inserted at the top.
	pub fn remove_line(&mut self, row: i8) -> Line {
		let line = self.field[row as usize];
		for i in row as usize..MAX_HEIGHT - 1 {
			self.field[i] = self.field[i + 1];
		}
		line
	}
	/// Inserts a line.
	///
	/// The existing lines are shifted up and the top line that got bumped out is returned.
	pub fn insert_line(&mut self, row: i8, line: Line) -> Line {
		let old = self.field[self.height() as usize - 1];
		for i in (row as usize..self.height() as usize - 1).rev() {
			self.field[i + 1] = self.field[i];
		}
		self.field[row as usize] = line;
		old
	}
}

impl Well {
	pub fn count_holes(&self) -> i32 {
		let mut well = *self;
		well.flood_fill();
		self.width as i32 * self.height as i32 - self.count_blocks() as i32
	}
	/// Returns the number of blocks in the field.
	pub fn count_blocks(&self) -> u32 {
		self.lines().iter().map(|&line| line.count_ones()).sum()
	}
	/// Flood fills the field.
	///
	/// The flood starts from the top-center of the field.
	pub fn flood_fill(&mut self) {
		let y = self.height - 1;
		let x = self.width as usize / 2;
		self._flood_fill(y as usize, 1 << x);
	}
	fn _flood_fill(&mut self, y: usize, x: Line) {
		// Find the left edge
		let mut left = x;
		while left > 1 && self.field[y] & (left >> 1) == 0 {
			left >>= 1;
		}
		let left = left;
		// Find the right edge (+ 1)
		let mut right = x;
		let end = 1 << self.width as usize;
		while right < end && self.field[y] & right == 0 {
			right <<= 1;
		}
		let right = right;
		// Mask all the blocks between left and right
		let mask = right - left;
		// println!("y:{} field:{:010b}, x:{:010b} left:{:010b} right:{:010b} mask:{:010b}", y, self.field[y], x, left, right, mask);
		self.field[y] |= mask;
		// Recursively try one row higher, since this is rare test it before hand
		if y < (self.height - 1) as usize && self.field[y + 1] & mask != mask {
			let mut it = left;
			while it < right {
				if self.field[y + 1] & it == 0 {
					self._flood_fill(y + 1, it);
				}
				it <<= 1;
			}
		}
		// Recursively try one row lower
		if y > 0 {
			let mut it = left;
			while it < right {
				if self.field[y - 1] & it == 0 {
					self._flood_fill(y - 1, it);
				}
				it <<= 1;
			}
		}
	}
}

/// Errors when parsing a well from text.
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
				width: width as i8,
				height: height as i8,
				_pad: 0,
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
		// let mut bg = " ";
		for &row in self.field[0..self.height() as usize].iter().rev() {
			f.write_str("|")?;
			let mut mask = 0x1;
			for _ in 0..self.width() {
				let graphic = if (row & mask) != 0 { MINOS_STR } else { " " };
				f.write_str(graphic)?;
				mask <<= 1;
			}
			f.write_str("|\n")?;
			// if bg == " " {
			// 	bg = "_";
			// }
			// else if bg == "_" {
			// 	bg = ".";
			// }
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

		well.etch(p1);
		well.etch(p2);
		well.etch(p3);
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
	fn test_player() {
		let well = well();
		// Within the field bounds
		assert!(!well.test_player(Player::new(Piece::S, Rot::Zero, Point::new(-1, 3))));
		assert!(!well.test_player(Player::new(Piece::J, Rot::Three, Point::new(5, 2))));
		// Clip left wall
		assert!(well.test_player(Player::new(Piece::S, Rot::Zero, Point::new(-2, 3))));
		// Clip with existing pieces
		assert!(well.test_player(Player::new(Piece::I, Rot::Two, Point::new(2, 3))));
		// Clip right wall
		assert!(well.test_player(Player::new(Piece::O, Rot::One, Point::new(9, 1))));
		// Clip the bottom
		assert!(well.test_player(Player::new(Piece::J, Rot::Three, Point::new(5, 1))));
	}

	#[test]
	fn lines() {
		let mut well = well();

		let base1 = well.line(0);
		let top1 = well.line(3);
		assert_eq!(0b1000111011, base1);
		assert_eq!(0b1000000000, top1);

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

		let insert1 = well.insert_line(0, removed1);
		assert_eq!(0b0000000000, insert1);
		assert_eq!("|          |\n\
		            |         □|\n\
		            |□□   □   □|\n\
		            |         □|\n\
		            +----------+", format!("{}", well));

		let insert2 = well.insert_line(1, removed2);
		assert_eq!(0b0000000000, insert2);
		assert_eq!("|         □|\n\
		            |□□   □   □|\n\
		            |□□ □□□   □|\n\
		            |         □|\n\
		            +----------+", format!("{}", well));

		let erased1 = well.set_line(0, 0b1000001111);
		assert_eq!(0b1000000000, erased1);
		assert_eq!("|         □|\n\
		            |□□   □   □|\n\
		            |□□ □□□   □|\n\
		            |□□□□     □|\n\
		            +----------+", format!("{}", well));
	}

	#[test]
	fn flood_fill() {
		let mut well = Well::from_data(10, &[
			0b0000000011,
			0b0000011011,
			0b0001100100,
			0b1000000100,
			0b0100101000,
			0b0011010000,
		]);
		println!("\n{}", well);
		well.flood_fill();
		println!("{}", well);
		let result = Well::from_data(10, &[
			0b1111111111,
			0b1111111111,
			0b1111111100,
			0b1111111100,
			0b0111111000,
			0b0011010000,
		]);
		assert_eq!(result, well);
	}
}
