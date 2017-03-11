/*!
Point.
*/

/// Point.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Point {
	pub x: i8,
	pub y: i8,
}

impl Point {
	pub fn new(x: i8, y: i8) -> Point {
		Point {
			x: x,
			y: y,
		}
	}
}
