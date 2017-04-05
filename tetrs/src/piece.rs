
use ::std::mem;

pub struct Sprite {
	pub pix: [u8; 4],
}

/// All the valid tetrominoes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Piece { O, I, S, Z, L, J, T }

impl ::rand::Rand for Piece {
	fn rand<R: ::rand::Rng>(rng: &mut R) -> Piece {
		let entropy = rng.next_u32();
		unsafe { mem::transmute((entropy % 7) as u8) }
	}
}
