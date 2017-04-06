
use ::std::mem;

/// Piece sprite.
///
/// The sprite pixels are 4x4 with only the low nibble used.
pub struct Sprite {
	pub pix: [u8; 4],
}

/// All the valid tetrominoes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Piece {
	/// The `O` tetromino.
	///
	/// ```text
	/// □□
	/// □□
	/// ```
	O,
	/// The `I` tetromino.
	///
	/// ```text
	/// □□□□
	/// ```
	I,
	/// The `S` tetromino.
	///
	/// ```text
	///  □□
	/// □□
	/// ```
	S,
	/// The `Z` tetromino.
	///
	/// ```text
	/// □□
	///  □□
	/// ```
	Z,
	/// The `L` tetromino.
	///
	/// ```text
	/// □
	/// □
	/// □□
	/// ```
	L,
	/// The `J` tetromino.
	///
	/// ```text
	///  □
	///  □
	/// □□
	/// ```
	J,
	/// The `T` tetromino.
	///
	/// ```text
	/// □□□
	///  □
	/// ```
	T,
}

impl ::rand::Rand for Piece {
	fn rand<R: ::rand::Rng>(rng: &mut R) -> Piece {
		let entropy = rng.next_u32();
		unsafe { mem::transmute((entropy % 7) as u8) }
	}
}
