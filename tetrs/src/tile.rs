/*!
Well graphics.
*/

use ::std::mem;

use ::Piece;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TileTy {
	/// This tile is a live player.
	Player,
	/// This tile is a location where the piece will land if dropped.
	Ghost,
	/// This tile is a block in the playing field.
	Field,
	/// This tile is a background block.
	Background,
}

/// Graphics tile.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tile(u8);
impl Tile {
	pub fn from(ty: TileTy, part: u8, piece: Option<Piece>) -> Tile {
		let ty = ty as u8;
		let piece = piece.map(|p| p as u8).unwrap_or(0b111);
		Tile(ty << 6 | piece << 3 | part)
	}
	pub fn tile_ty(self) -> TileTy {
		unsafe { mem::transmute(self.0 >> 6) }
	}
	pub fn part(self) -> u8 {
		self.0 & 0b00_000_111
	}
	pub fn piece(self) -> Option<Piece> {
		match (self.0 & 0b00_111_000) >> 3 {
			0b000 => Some(Piece::O),
			0b001 => Some(Piece::I),
			0b010 => Some(Piece::S),
			0b011 => Some(Piece::Z),
			0b100 => Some(Piece::L),
			0b101 => Some(Piece::J),
			0b110 => Some(Piece::T),
			_ => None,
		}
	}
}
impl From<u8> for Tile {
	fn from(byte: u8) -> Tile {
		Tile(byte)
	}
}
impl Into<u8> for Tile {
	fn into(self) -> u8 {
		self.0
	}
}

pub const TILE_BG0: Tile = Tile(0b11_000_000);
pub const TILE_BG1: Tile = Tile(0b11_001_000);
pub const TILE_BG2: Tile = Tile(0b11_010_000);
