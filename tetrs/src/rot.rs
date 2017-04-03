/*!
Piece rotation.
*/

use ::std::mem;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Rot { Zero, One, Two, Three }

impl Rot {
	pub fn cw(self) -> Rot { unsafe {
		mem::transmute((self as u8).wrapping_add(1) & 3)
	}}
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
		assert_eq!(Rot::One, Rot::Zero.cw());
		assert_eq!(Rot::Three, Rot::Zero.ccw());
	}
}
