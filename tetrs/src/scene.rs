/*!
Well scene.
*/

use ::{Player, Tile, TileTy, TILE_BG0, TILE_BG1, TILE_BG2, MAX_HEIGHT, MAX_WIDTH};

/// Well scene.
///
/// The scene tracks the visual tiles in the well.
///
/// This allows a client to visualize the well based on which pieces were dropped here
/// without requiring any of the game logic to work with this less efficient data structure.
#[derive(Clone, Debug)]
pub struct Scene {
	width: i8,
	height: i8,
	tiles: [[Tile; MAX_WIDTH]; MAX_HEIGHT]
}
impl Scene {
	pub fn new(width: i8, height: i8) -> Scene {
		let mut bg = [[TILE_BG0; MAX_WIDTH]; MAX_HEIGHT];
		bg[height as usize - 2] = [TILE_BG1; MAX_WIDTH];
		bg[height as usize - 1] = [TILE_BG2; MAX_WIDTH];
		Scene {
			width: width,
			height: height,
			tiles: bg,
		}
	}
	pub fn width(&self) -> i8 {
		self.width
	}
	pub fn height(&self) -> i8 {
		self.height
	}
	pub fn line(&self, row: i8) -> &[Tile] {
		&self.tiles[(self.height - 1 - row) as usize][..self.width as usize]
	}
	/// Draws the player and its ghost into the scene.
	pub fn draw(&mut self, player: Player, tile_ty: TileTy) {
		// Get the unperturbed mesh
		let mesh = player.piece.mesh().data[player.rot as u8 as usize];
		let mut part_id = 0;
		// Render the tiles to the scene
		for y in 0..4 {
			let mut mask = mesh[y as usize];
			for x in 0..4 {
				if mask & 1 != 0 {
					let row = player.pt.y - y;
					let col = player.pt.x + x;
					if col >= 0 && col < self.width && row >= 0 && row < self.height {
						let tile = Tile::from(tile_ty, part_id, Some(player.piece));
						self.tiles[row as usize][col as usize] = tile;
					}
					part_id += 1;
				}
				mask >>= 1;
			}
		}
	}
	pub fn remove_line(&mut self, row: i8) {
		let top = (self.height - 2) as usize;
		let _ = self.tiles[row as usize..top];
		for i in row as usize..top {
			self.tiles[i] = self.tiles[i + 1];
		}
		self.tiles[top] = [TILE_BG0; MAX_WIDTH];
		self.fix_bg();
	}
	fn fix_bg(&mut self) {
		let height = self.height as usize;
		for tile in self.tiles[height - 1].iter_mut() {
			if tile.tile_ty() == TileTy::Background {
				*tile = TILE_BG2;
			}
		}
		for tile in self.tiles[height - 2].iter_mut() {
			if tile.tile_ty() == TileTy::Background {
				*tile = TILE_BG1;
			}
		}
		for tile in self.tiles[height - 3].iter_mut() {
			if tile.tile_ty() == TileTy::Background {
				*tile = TILE_BG0;
			}
		}
	}
}
