
use ::std::mem;

/// Represents the shapes of a piece.
pub struct Mesh {
	pub data: [Sprite; 4],
}

pub struct Sprite {
	pub pix: [u8; 4],
}

/// All the valid tetrominoes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Piece { O, I, S, Z, L, J, T }

impl Piece {
	/// Returns the Mesh data for the given piece.
	pub fn mesh(self) -> &'static Mesh {
		&DATA[self as u8 as usize]
	}
}
impl ::rand::Rand for Piece {
	fn rand<R: ::rand::Rng>(rng: &mut R) -> Piece {
		let entropy = rng.next_u32();
		unsafe { mem::transmute((entropy % 7) as u8) }
	}
}

macro_rules! b {
	(____) => (0b0000);
	(___X) => (0b1000);
	(__X_) => (0b0100);
	(__XX) => (0b1100);
	(_X__) => (0b0010);
	(_X_X) => (0b1010);
	(_XX_) => (0b0110);
	(_XXX) => (0b1110);
	(X___) => (0b0001);
	(X__X) => (0b1001);
	(X_X_) => (0b0101);
	(X_XX) => (0b1101);
	(XX__) => (0b0011);
	(XX_X) => (0b1011);
	(XXX_) => (0b0111);
	(XXXX) => (0b1111);
}
macro_rules! data {
	(
		$a11:tt $a12:tt $a13:tt $a14:tt
		$a21:tt $a22:tt $a23:tt $a24:tt
		$a31:tt $a32:tt $a33:tt $a34:tt
		$a41:tt $a42:tt $a43:tt $a44:tt
	) => {
		[
			Sprite { pix: [b!($a11), b!($a21), b!($a31), b!($a41)] },
			Sprite { pix: [b!($a12), b!($a22), b!($a32), b!($a42)] },
			Sprite { pix: [b!($a13), b!($a23), b!($a33), b!($a43)] },
			Sprite { pix: [b!($a14), b!($a24), b!($a34), b!($a44)] },
		]
	};
}

static DATA: [Mesh; 7] = [
	// The O piece
	Mesh { data: data![
		____ ____ ____ ____
		_XX_ _XX_ _XX_ _XX_
		_XX_ _XX_ _XX_ _XX_
		____ ____ ____ ____
	]},
	// The I piece
	Mesh { data: data![
		____ __X_ ____ _X__
		XXXX __X_ ____ _X__
		____ __X_ XXXX _X__
		____ __X_ ____ _X__
	]},
	// The S piece
	Mesh { data: data![
		__XX __X_ ____ _X__
		_XX_ __XX __XX _XX_
		____ ___X _XX_ __X_
		____ ____ ____ ____
	]},
	// The Z piece
	Mesh { data: data![
		_XX_ ___X ____ __X_
		__XX __XX _XX_ _XX_
		____ __X_ __XX _X__
		____ ____ ____ ____
	]},
	// The L piece
	Mesh { data: data![
		___X __X_ ____ _XX_
		_XXX __X_ _XXX __X_
		____ __XX _X__ __X_
		____ ____ ____ ____
	]},
	// The J piece
	Mesh { data: data![
		_X__ __XX ____ __X_
		_XXX __X_ _XXX __X_
		____ __X_ ___X _XX_
		____ ____ ____ ____
	]},
	// The T piece
	Mesh { data: data![
		__X_ __X_ ____ __X_
		_XXX __XX _XXX _XX_
		____ __X_ __X_ __X_
		____ ____ ____ ____
	]},
];
