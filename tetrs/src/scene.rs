/*!
*/

use ::{Player, Tile, TileTy, TILE_BG0, TILE_BG1, TILE_BG2, MAX_HEIGHT, MAX_WIDTH};

#[derive(Clone, Debug)]
pub struct Scene {
	width: i32,
	height: i32,
	tiles: [[Tile; MAX_WIDTH]; MAX_HEIGHT]
}
impl Scene {
	pub fn new(width: i32, height: i32) -> Scene {
		let mut bg = [[TILE_BG0; MAX_WIDTH]; MAX_HEIGHT];
		bg[height as usize - 2] = [TILE_BG1; MAX_WIDTH];
		bg[height as usize - 1] = [TILE_BG2; MAX_WIDTH];
		Scene {
			width: width,
			height: height,
			tiles: bg,
		}
	}
	pub fn width(&self) -> i32 {
		self.width
	}
	pub fn height(&self) -> i32 {
		self.height
	}
	pub fn line(&self, row: i32) -> &[Tile] {
		&self.tiles[(self.height - 1 - row) as usize][..self.width as usize]
	}
	pub fn render(&mut self, player: &Player, tile_ty: TileTy) {
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
	pub fn remove_line(&mut self, row: i32) {
		let top = (self.height - 2) as usize;
		let _ = self.tiles[row as usize..top];
		for i in row as usize..top - 1 {
			self.tiles[i] = self.tiles[i + 1];
		}
		self.tiles[top] = [TILE_BG0; MAX_WIDTH];
	}
}
