/*!
Playing field.
*/

use ::std::{fmt};
use ::std::str::{FromStr};

use ::{Point, Sprite};

/// Maximum well height.
///
// If this is changed, don't forget to update the documentation for `Well::new`.
///
/// Note that the absolute limit is about `123` (max value for `i8` - `4` for padding).
pub const MAX_HEIGHT: usize = 23;

/// Maxium well width.
///
// If this is changed, don't forget to update the documentation for `Well::new`.
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
	field: [Line; MAX_HEIGHT],
}

const MINOS_STR: &'static str = "□";

impl Well {
	/// Creates an empty well with the given dimensions.
	///
	/// # Panics
	///
	/// The width must be ∈ [4, 16] and the height must be ∈ [4, 23].
	pub fn new(width: i8, height: i8) -> Well {
		assert!(width >= 4 && width <= MAX_WIDTH as i8, "width must be ∈ [4, {}]", MAX_WIDTH);
		assert!(height >= 4 && height <= MAX_HEIGHT as i8, "height must be ∈ [4, {}]", MAX_HEIGHT);
		Well {
			width: width,
			height: height,
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
	/// Tests if the sprite collides with with the well.
	pub fn test(&self, sprite: &Sprite, pt: Point) -> bool {
		// Early reject out of bounds
		if pt.x < (0 - 4) || pt.x >= self.width || pt.y < 0 {
			return true;
		}
		if pt.y >= self.height + 4 {
			return false;
		}

		// For clipping left/right walls
		let line_mask = if pt.x < 0 {
			self.line_mask() << (-pt.x) as usize
		}
		else {
			self.line_mask() >> pt.x as usize
		};

		// The compiler actually unrolls and splits this loop, pretty slick :)
		for y in 0..4 {
			// Check if part is sticking out of a wall
			if (sprite.pix[y as usize] as Line) & !line_mask != 0 {
				return true;
			}
			let row = pt.y - y;
			// If this row is below the floor
			if row < 0 {
				if sprite.pix[y as usize] != 0 {
					return true;
				}
			}
			// If this row is below the ceiling
			else if row < self.height {
				// Render the sprite for this line
				let cg_line = if pt.x < 0 {
					(sprite.pix[y as usize] as Line) >> (-pt.x) as usize
				}
				else {
					(sprite.pix[y as usize] as Line) << pt.x as usize
				};
				if cg_line & self.field[row as usize] != 0 {
					return true;
				}
			}
		}
		return false;
	}
	/// Tests a list of kicks and returns the first point where the sprite doesn't collide with the well.
	///
	/// Results in `None` if all kicks collide with the well.
	#[inline]
	pub fn wall_kick(&self, sprite: &Sprite, kicks: &[Point], pt: Point) -> Option<Point> {
		kicks.iter()
			.map(|&offset| pt + offset)
			.find(|&pt| !self.test(sprite, pt))
	}
	/// Traces the sprite down and returns the lowest point where it does not collide with the well.
	pub fn trace_down(&self, sprite: &Sprite, mut pt: Point) -> Point {
		loop {
			let next = Point::new(pt.x, pt.y - 1);
			if self.test(sprite, next) {
				return pt;
			}
			pt = next;
		}
	}
	/// Etches the sprite into the well.
	pub fn etch(&mut self, sprite: &Sprite, pt: Point) {
		// Etch the sprite into the field
		for y in 0..4 {
			// Clip the affected row to the field
			let row = pt.y - y;
			if row >= 0 && row < self.height {
				// Render the sprite for this line
				let line_mask = if pt.x < 0 {
					(sprite.pix[y as usize] as Line) >> (-pt.x) as usize
				}
				else {
					(sprite.pix[y as usize] as Line) << pt.x as usize
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
	/// Counts the number of holes.
	///
	/// A hole is defined as an empty block that is not reachable from the top of the well.
	pub fn count_holes(&self) -> i32 {
		let mut well = *self;
		let seed = Point::new(self.width >> 1, self.height - 1);
		well.flood_fill(seed);
		well.width as i32 * well.height as i32 - well.count_blocks() as i32
	}
	/// Returns the number of blocks in the field.
	pub fn count_blocks(&self) -> u32 {
		self.lines().iter().map(|&line| line.count_ones()).sum()
	}
	/// Flood fills the field from the given seeding point.
	pub fn flood_fill(&mut self, seed: Point) {
		self._flood_fill(seed.y as usize, 1 << seed.x as usize);
	}
	fn _flood_fill(&mut self, y: usize, x: Line) {
		// Bounds check it early (optimizer is dumb...)
		//let _ = self.lines()[y];
		// WARNING! UB if `width >= sizeof(Line) * 8`...
		let end = (1u32 << self.width as usize) as Line;
		// Since the top of the well is most likely open, optimize for this case
		let (left, right) = if self.field[y] == 0 {
			(1, end)
		}
		else {
			// Find the left edge
			let mut left = x;
			while left > 1 && self.field[y] & (left >> 1) == 0 {
				left >>= 1;
			}
			// Find the right edge (+ 1)
			let mut right = x;
			while right < end && self.field[y] & right == 0 {
				right <<= 1;
			}
			(left, right)
		};
		// Mask all the blocks between left and right
		let mask = right - left;
		self.field[y] |= mask;

		// Let's do some tail call optimization first
		if y >= 1 {
			if self.field[y - 1] & mask == 0 {
				return self._flood_fill(y - 1, left);
			}
			// Recursively flood the rest
			let mut it = left;
			while it < right {
				if self.field[y - 1] & it == 0 {
					self._flood_fill(y - 1, it);
				}
				it <<= 1;
			}
		}
		// Since we're flooding top to bottom first, this case is considerably more rare
		if y + 1 < self.height as usize && self.field[y + 1] & mask != mask {
			let mut it = left;
			while it < right {
				if self.field[y + 1] & it == 0 {
					self._flood_fill(y + 1, it);
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
	use ::{Player, Piece, Rot, Point, test_player};

	fn well() -> Well {
		let mut well = Well::new(10, 4);

		let p1 = Player::new(Piece::L, Rot::Zero, Point::new(2, 1));
		let p2 = Player::new(Piece::O, Rot::Zero, Point::new(-1, 2));
		let p3 = Player::new(Piece::I, Rot::Right, Point::new(7, 3));

		well.etch(p1.sprite(), p1.pt);
		well.etch(p2.sprite(), p2.pt);
		well.etch(p3.sprite(), p3.pt);
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
	fn test_player_test() {
		let well = well();
		// Within the field bounds
		assert!(!test_player(&well, Player::new(Piece::S, Rot::Zero, Point::new(-1, 3))));
		assert!(!test_player(&well, Player::new(Piece::J, Rot::Left, Point::new(5, 2))));
		// Clip left wall
		assert!(test_player(&well, Player::new(Piece::S, Rot::Zero, Point::new(-2, 3))));
		// Clip with existing pieces
		assert!(test_player(&well, Player::new(Piece::I, Rot::Two, Point::new(2, 3))));
		// Clip right wall
		assert!(test_player(&well, Player::new(Piece::O, Rot::Right, Point::new(9, 1))));
		// Clip the bottom
		assert!(test_player(&well, Player::new(Piece::J, Rot::Left, Point::new(5, 1))));
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
		well.flood_fill(Point::new(5, 5));
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
