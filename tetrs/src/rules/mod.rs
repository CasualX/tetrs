/*!
Customize the rules for the tetris game.
*/

use ::{Piece, Rot, Point, srs_cw, srs_ccw};

/// Tetris rule customization.
pub trait Rules: Copy {
	/// The 4x4 mesh for the given piece and rotation.
	fn piece_mesh(&self, piece: Piece, rot: Rot) -> [u8; 4];
	fn rotate_cw_offsets(&self, piece: Piece, rot: Rot) -> &[Point];
	fn rotate_ccw_offsets(&self, piece: Piece, rot: Rot) -> &[Point];
}

#[derive(Copy, Clone)]
pub struct TheRules;
impl Rules for TheRules {
	fn piece_mesh(&self, piece: Piece, rot: Rot) -> [u8; 4] {
		DATA[piece as u8 as usize].data[rot as u8 as usize]
	}
	fn rotate_cw_offsets(&self, piece: Piece, rot: Rot) -> &[Point] {
		srs_cw(piece, rot)
	}
	fn rotate_ccw_offsets(&self, piece: Piece, rot: Rot) -> &[Point] {
		srs_ccw(piece, rot)
	}
}

//----------------------------------------------------------------

struct Mesh {
	pub data: [[u8; 4]; 4],
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
			[b!($a11), b!($a21), b!($a31), b!($a41)],
			[b!($a12), b!($a22), b!($a32), b!($a42)],
			[b!($a13), b!($a23), b!($a33), b!($a43)],
			[b!($a14), b!($a24), b!($a34), b!($a44)],
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

//----------------------------------------------------------------