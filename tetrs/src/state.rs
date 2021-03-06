
use ::{Player, Well, Piece, Rot, Point, Scene, TileTy, srs_cw, srs_ccw};

/// Game state of player and well.
#[derive(Clone, Debug)]
pub struct State {
	player: Option<Player>,
	well: Well,
	scene: Scene,
}

impl State {
	/// Creates a new game state.
	///
	/// Don't forget to spawn a player!
	pub fn new(width: i8, height: i8) -> State {
		State {
			player: None,
			well: Well::new(width, height),
			scene: Scene::new(width, height),
		}
	}
	/// Creates a new game state from existing well.
	pub fn with_well(well: Well) -> State {
		let scene = Scene::new(well.width(), well.height());
		State {
			player: None,
			well: well,
			scene: scene,
		}
	}
	/// Returns the current player.
	pub fn player(&self) -> Option<&Player> {
		self.player.as_ref()
	}
	/// Sets the current player.
	pub fn set_player(&mut self, player: Player) {
		self.player = Some(player)
	}
	/// Returns the well.
	pub fn well(&self) -> &Well {
		&self.well
	}
	/// Moves the player one block to the left.
	///
	/// Does nothing and returns `false` if no player or no space to move left.
	pub fn move_left(&mut self) -> bool {
		let player = match self.player { Some(pl) => pl, None => return false };
		let next = player.move_left();
		if !test_player(&self.well, next) {
			self.player = Some(next);
			true
		}
		else {
			false
		}
	}
	/// Moves the player one block to the right.
	///
	/// Does nothing and returns `false` if no player or no space to move right.
	pub fn move_right(&mut self) -> bool {
		let player = match self.player { Some(pl) => pl, None => return false };
		let next = player.move_right();
		if !test_player(&self.well, next) {
			self.player = Some(next);
			true
		}
		else {
			false
		}
	}
	/// Rotates the player clockwise.
	///
	/// Does nothing and returns `false` if no player or no space to rotate clockwise.
	///
	/// If there's not enough space a wall kick is attempted.
	pub fn rotate_cw(&mut self) -> bool {
		let player = match self.player { Some(pl) => pl, None => return false };
		let next = srs_cw(&self.well, player);
		self.player = Some(next);
		player != next
	}
	/// Rotates the player counter-clockwise.
	///
	/// Does nothing and returns `false` if no player or no space to rotate counter-clockwise.
	///
	/// If there's not enough space a wall kick is attempted.
	pub fn rotate_ccw(&mut self) -> bool {
		let player = match self.player { Some(pl) => pl, None => return false };
		let next = srs_ccw(&self.well, player);
		self.player = Some(next);
		player != next
	}
	/// Drops the player down one block.
	///
	/// Returns `false` if no player and locks the player if no space to drop down.
	pub fn soft_drop(&mut self) -> bool {
		let player = match self.player { Some(pl) => pl, None => return false };
		let next = player.move_down();
		if !test_player(&self.well, next) {
			self.player = Some(next);
			true
		}
		else {
			// If we get stuck, lock the player here
			self.lock();
			false
		}
	}
	/// Drops and locks the player all the way down.
	///
	/// Returns `false` if no player.
	pub fn hard_drop(&mut self) -> bool {
		if let Some(player) = self.player {
			self.player = Some(trace_down(&self.well, player));
			self.lock();
			true
		}
		else {
			false
		}
	}
	/// Applies gravity to the player.
	///
	/// Returns `false` if no player and locks the player if no space to drop down.
	pub fn gravity(&mut self) -> bool {
		self.soft_drop()
	}
	/// Check for line clears.
	///
	/// The callback is called for every cleared line with the row being cleared from bottom to top.
	pub fn clear_lines<F>(&mut self, mut f: F) -> i32 where F: FnMut(i32) {
		let mut cleared = 0;
		let line_mask = self.well.line_mask();
		let mut row = 0;
		while row < self.well.height() {
			if self.well.line(row) == line_mask {
				f(row as i32 + cleared);
				self.well.remove_line(row);
				self.scene.remove_line(row);
				cleared += 1;
			}
			else {
				row += 1;
			}
		}
		cleared
	}
	/// Etch the player to the well and kill it.
	pub fn lock(&mut self) {
		if let Some(pl) = self.player {
			self.well.etch(pl.sprite(), pl.pt);
			self.scene.draw(pl, TileTy::Field);
			self.player = None;
		}
	}
	/// Spawns a new player with the given piece.
	///
	/// The spawning location is at the top of the well, centered horizontally with zero rotation.
	///
	/// Returns `false` if the spawned piece overlaps with a block in the well.
	pub fn spawn(&mut self, piece: Piece) -> bool {
		self.player = Some(Player {
			piece: piece,
			rot: Rot::Zero,
			pt: Point {
				x: self.well.width() / 2 - 2,
				y: self.well.height() - (piece != Piece::O && piece != Piece::I) as i8,
			},
		});
		test_player(&self.well, self.player.unwrap())
	}
	/// Tests if the well extends to the top 2 lines.
	pub fn is_game_over(&self) -> bool {
		let lines = self.well.lines();
		let height = self.well.height() as usize;
		lines[height - 1] != 0 || lines[height - 2] != 0
	}
	pub fn scene(&self) -> Scene {
		let mut scene = self.scene.clone();
		if let Some(&player) = self.player() {
			// Draw the ghost where the player will fall
			let ghost = trace_down(&self.well, player);
			scene.draw(ghost, TileTy::Ghost);
			// Draw the player
			scene.draw(player, TileTy::Player);
		}
		scene
	}
}

pub fn test_player(well: &Well, player: Player) -> bool {
	let sprite = player.sprite();
	well.test(sprite, player.pt)
}
pub fn trace_down(well: &Well, player: Player) -> Player {
	let sprite = player.sprite();
	let pt = well.trace_down(sprite, player.pt);
	Player::new(player.piece, player.rot, pt)
}
