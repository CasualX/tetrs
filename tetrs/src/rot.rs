/*!
Piece rotation.
*/

use ::std::mem;

/// Rotation state of a piece.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Rot {
	/// Spawn state.
	Zero,
	/// Clockwise rotation ("right") from spawn.
	Right,
	/// 2 Successive rotations in either direction from spawn.
	Two,
	/// Counter-clockwise ("left") rotation from spawn.
	Left,
}

impl Rot {
	/// Rotate clockwise.
	pub fn cw(self) -> Rot { unsafe {
		mem::transmute((self as u8).wrapping_add(1) & 3)
	}}
	/// Rotate counter-clockwise.
	pub fn ccw(self) -> Rot { unsafe {
		mem::transmute((self as u8).wrapping_sub(1) & 3)
	}}
}

impl From<u8> for Rot {
	fn from(val: u8) -> Rot { unsafe {
		mem::transmute(val & 3)
	}}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rotate() {
		assert_eq!(Rot::Right, Rot::Zero.cw());
		assert_eq!(Rot::Left, Rot::Zero.ccw());
	}
}
