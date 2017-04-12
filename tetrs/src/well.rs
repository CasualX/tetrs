/*!
Playing field.
*/

use ::std::{fmt};
use ::std::str::{FromStr};

use ::{Point, Sprite};

/// Row in the well.
///
/// The well represents its internal structure in bit masks.
//
// Keep in sync with `SIZE_OF_WIDTH` and `MAX_WIDTH`.
pub type Line = u16;
const SIZE_OF_WIDTH: usize = 16;

/// Maximum well height.
///
/// The well uses a fixed size array to store its field making it very cheap to copy.
// This height was chosen to make the size of `Well` equal to 80 bytes, which is 5 times size of xmm register.
//
// If this is changed, don't forget to update the documentation for `Well::new`.
//
// Note that the absolute limit is about `123` (max value for `i8` - `4` for padding).
pub const MAX_HEIGHT: usize = 23;

/// Maximum well width.
///
// If this is changed, don't forget to update the documentation for `Well::new`.
//
// This should be equal to `size_of(Line) - 4`.
// Subtract 4 is needed to avoid handling some sprite test edge cases (sprites are 4x4).
pub const MAX_WIDTH: usize = 12;

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
	/// The width must be ∈ [4, 12] and the height must be ∈ [4, 23].
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
	/// Note that the input lines are in 'visual' order. Internally the lines are stored bottom line first.
	///
	/// # Panics
	///
	/// No minos may be found outside the well's width.
	pub fn from_data(width: i8, lines: &[Line]) -> Well {
		let mut well = Well::new(width, lines.len() as i8);
		let shift = SIZE_OF_WIDTH as usize - width as usize;
		for (lhs, &rhs) in Iterator::zip(well.field[..lines.len()].iter_mut(), lines.iter().rev()) {
			*lhs = rhs << shift;
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
	pub fn col_range(&self) -> ColRange {
		ColRange {
			start: 1 << (SIZE_OF_WIDTH - 1),
			end: 1 << (SIZE_OF_WIDTH - self.width() as usize - 1),
		}
	}
	fn render(sprite: &Sprite, x: i8) -> [Line; 4] {
		let mut result = [0; 4];
		for y in 0..4 {
			result[y] = (sprite.pix[y] as Line).rotate_right((x + 4) as u32);
		}
		result
	}
	/// Tests if the sprite collides with with the well.
	pub fn test(&self, sprite: &Sprite, pt: Point) -> bool {
		// Early reject out of bounds
		if pt.x <= -4 || pt.x >= self.width || pt.y < 0 {
			return true;
		}
		if pt.y >= self.height + 4 {
			return false;
		}

		// Render the sprite
		let sprite = Self::render(sprite, pt.x);
		let line_mask = self.line_mask();

		for y in 0..4 {
			// Check if part is sticking out of a wall
			if sprite[y as usize] & !line_mask != 0 {
				return true;
			}
			let row = pt.y - y;
			// If this row is below the floor
			if row < 0 {
				if sprite[y as usize] != 0 {
					return true;
				}
			}
			// If this row is below the ceiling
			else if row < self.height {
				// Render the sprite for this line
				if sprite[y as usize] & self.field[row as usize] != 0 {
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
		let sprite = Self::render(sprite, pt.x);
		// Etch the sprite into the field
		for y in 0..4 {
			// Clip the affected row to the field
			let row = pt.y - y;
			if row >= 0 && row < self.height {
				self.field[row as usize] |= sprite[y as usize];
			}
		}
	}
	/// Gets a line with all columns set.
	pub fn line_mask(&self) -> Line {
		let shift = SIZE_OF_WIDTH - self.width() as usize;
		!((1 << shift) - 1)
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
		let x = self.col_range().nth(seed.x as usize).unwrap();
		self._flood_fill(seed.y as usize, x);
	}
	fn _flood_fill(&mut self, y: usize, x: Line) {
		// Bounds check it early (optimizer is dumb...)
		//let _ = self.lines()[y];
		let mut range = self.col_range();
		// Since the top of the well is most likely open, optimize for this case
		if self.field[y] != 0 {
			// Find the left edge
			let mut left = x;
			while left < range.start && self.field[y] & (left << 1) == 0 {
				left <<= 1;
			}
			// Find the right edge (+ 1)
			let mut right = x;
			while right > range.end && self.field[y] & right == 0 {
				right >>= 1;
			}
			range = ColRange {
				start: left,
				end: right,
			};
		}
		// Mask all the blocks between left and right
		let mask = range.mask();
		self.field[y] |= mask;

		// Let's do some tail call optimization first
		if y >= 1 {
			if self.field[y - 1] & mask == 0 {
				return self._flood_fill(y - 1, range.start);
			}
			// Recursively flood the rest
			for it in range.clone() {
				if self.field[y - 1] & it == 0 {
					self._flood_fill(y - 1, it);
				}
			}
		}
		// Since we're flooding top to bottom first, this case is considerably more rare
		if y + 1 < self.height as usize && self.field[y + 1] & mask != mask {
			for it in range.clone() {
				if self.field[y + 1] & it == 0 {
					self._flood_fill(y + 1, it);
				}
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
			for col_mask in self.col_range() {
				let graphic = if (row & col_mask) != 0 { MINOS_STR } else { " " };
				f.write_str(graphic)?;
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColRange {
	pub start: Line,
	pub end: Line,
}
impl ColRange {
	/// Creates a mask with all bits set in the range.
	pub fn mask(&self) -> Line {
		(self.start - self.end) << 1
	}
}
impl Iterator for ColRange {
	type Item = Line;
	fn next(&mut self) -> Option<Line> {
		if self.start != self.end {
			let it = self.start;
			self.start >>= 1;
			Some(it)
		}
		else {
			None
		}
	}
}
impl DoubleEndedIterator for ColRange {
	fn next_back(&mut self) -> Option<Line> {
		if self.start != self.end {
			self.end <<= 1;
			Some(self.end)
		}
		else {
			None
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn static_assert() {
		// These shouldn't need to be tests but for the lack of const fn.
		let size_of_line = ::std::mem::size_of::<Line>() * 8;
		assert_eq!(size_of_line, super::SIZE_OF_WIDTH);
		assert_eq!(size_of_line, MAX_WIDTH + 4);
		assert!(MAX_HEIGHT < 123);
	}

	#[test]
	fn render() {
		let sprite = Sprite { pix: [ 0b1000, 0b0111, 0b1110, 0b0001 ] };
		let rendered = Well::render(&sprite, 1);
		assert_eq!(rendered, [
			0b1000 << 11,
			0b0111 << 11,
			0b1110 << 11,
			0b0001 << 11,
		]);
	}

	#[test]
	fn col_range() {
		let well = Well::new(4, 4);
		let mut range = well.col_range();

		assert_eq!(     0b1111 << 12 , range.mask());

		assert_eq!(Some(0b1000 << 12), range.next());
		assert_eq!(     0b0111 << 12 , range.mask());

		assert_eq!(Some(0b0100 << 12), range.next());
		assert_eq!(     0b0011 << 12 , range.mask());

		assert_eq!(Some(0b0001 << 12), range.next_back());
		assert_eq!(     0b0010 << 12 , range.mask());

		assert_eq!(Some(0b0010 << 12), range.next());

		assert_eq!(None, range.next());
		assert_eq!(None, range.next_back());
	}

	#[test]
	fn etch() {
		#![allow(non_snake_case)]
		let mut well = Well::new(10, 4);
		let sprite_O = Sprite { pix: [ 0b0000, 0b0110, 0b0110, 0b0110 ] };
		let sprite_L = Sprite { pix: [ 0b0001, 0b0111, 0b0000, 0b0000 ] };
		let sprite_I = Sprite { pix: [ 0b0010, 0b0010, 0b0010, 0b0010 ] };
		well.etch(&sprite_O, Point::new(-1, 2));
		well.etch(&sprite_L, Point::new(2, 1));
		well.etch(&sprite_I, Point::new(7, 3));

		let result = Well::from_data(10, &[
			0b0000000001,
			0b0000000001,
			0b1100010001,
			0b1101110001,
		]);
		println!("\n{}", well);
		assert_eq!(result, well);
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
/*
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
*/
}
