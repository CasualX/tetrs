/*!
*/

#![allow(dead_code)]

extern crate tetrs;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2::GameControllerSubsystem;
use sdl2::controller::{self, GameController};
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::EventPump;
use sdl2::event::Event;

use std::time::Duration;
use std::thread;

//----------------------------------------------------------------

struct Sprites {
	pieces: [Rect; 8],
	ghost: Rect,
}
struct Map {
	field_x: i32,
	field_y: i32,
}

/// Width and height of a tile.
const TILE_SIZE: i32 = 20;

struct Graphics<'a> {
	renderer: Renderer<'a>,
	atlas: Texture,
	background: Texture,
	sprites: Sprites,
	map: Map,
}

fn draw(cg: &mut Graphics, scene: &tetrs::Scene) {
	cg.renderer.set_draw_color(Color::RGB(0, 0, 0));
	cg.renderer.clear();
	cg.renderer.copy(&cg.background, None, None).unwrap();

	draw_scene2(cg, scene);

	cg.renderer.present();
}

fn draw_scene1(cg: &mut Graphics, scene: &tetrs::Scene) {
	let width = scene.width() as i32;
	let height = scene.height() as i32;
	// Draw the scene
	for row in 0..height {
		let line = scene.line(row as i8);
		for col in 0..width {
			let tile = line[col as usize];
			let x = cg.map.field_x + col * TILE_SIZE;
			let y = cg.map.field_y + row * TILE_SIZE;
			let rect = Rect::new(x, y, TILE_SIZE as u32, TILE_SIZE as u32);

			use tetrs::TileTy::*;
			match tile.tile_ty() {
				Field | Player => {
					use tetrs::Piece;
					let color = match tile.piece() {
						Some(Piece::O) => Color::RGB(241, 238, 81),
						Some(Piece::I) => Color::RGB(83, 254, 248),
						Some(Piece::S) => Color::RGB(84, 254, 87),
						Some(Piece::Z) => Color::RGB(255, 85, 85),
						Some(Piece::L) => Color::RGB(254, 203, 36),
						Some(Piece::J) => Color::RGB(84, 85, 255),
						Some(Piece::T) => Color::RGB(255, 85, 254),
						None => Color::RGB(170, 170, 170),
					};
					cg.renderer.set_draw_color(color);
					cg.renderer.fill_rect(rect).unwrap();
				},
				Ghost => {
					cg.renderer.set_draw_color(Color::RGB(50, 50, 50));
					cg.renderer.fill_rect(rect).unwrap();
				},
				Background => {},
			};
		}
	}

	// Draw the columns
	cg.renderer.set_draw_color(Color::RGB(46, 46, 46));
	for col in 0..width + 1 {
		let x = cg.map.field_x + col * TILE_SIZE;
		let rect = Rect::new(x, cg.map.field_y, 1, (height * TILE_SIZE) as u32);
		cg.renderer.fill_rect(rect).unwrap();
	}

	// Draw the rows
	for row in 0..height + 1 {
		let y = cg.map.field_y + row * TILE_SIZE;
		let rect = Rect::new(cg.map.field_x, y, (width * TILE_SIZE) as u32, 1);
		cg.renderer.fill_rect(rect).unwrap();
	}
}

fn draw_scene2(cg: &mut Graphics, scene: &tetrs::Scene) {
	let width = scene.width() as i32;
	let height = scene.height() as i32;
	for row in 0..height {
		let line = scene.line(row as i8);
		for col in 0..width {
			let tile = line[col as usize];
			let x = cg.map.field_x + col * TILE_SIZE;
			let y = cg.map.field_y + row * TILE_SIZE;
			let rect = Rect::new(x, y, TILE_SIZE as u32, TILE_SIZE as u32);

			use tetrs::TileTy::*;
			match tile.tile_ty() {
				Field | Player => {
					let piece = tile.piece().map(|p| p as usize).unwrap_or(7);
					let sprite = cg.sprites.pieces[piece];
					cg.renderer.copy(&cg.atlas, Some(sprite), Some(rect)).unwrap();
				},
				Ghost => {
					cg.renderer.copy(&cg.atlas, Some(cg.sprites.ghost), Some(rect)).unwrap();
				},
				Background => {
				},
			};
		}
	}
}

//----------------------------------------------------------------

fn open_controller(gcs: &GameControllerSubsystem) -> Option<GameController> {
	let available = match gcs.num_joysticks() {
		Ok(n)  => n,
		Err(e) => panic!("can't enumerate joysticks: {}", e),
	};

	println!("{} joysticks available", available);

	let mut controller = None;

	// Iterate over all available joysticks and look for game
	// controllers.
	for id in 0..available {
		if gcs.is_game_controller(id) {
			println!("Attempting to open controller {}", id);

			match gcs.open(id) {
				Ok(c) => {
					// We managed to find and open a game controller,
					// exit the loop
					println!("Success: opened \"{}\"", c.name());
					controller = Some(c);
					break;
				},
				Err(e) => println!("failed: {:?}", e),
			}
		}
		else {
			println!("{} is not a game controller", id);
		}
	}

	// match controller {
	// 	Some(c) => {
	// 		println!("Controller mapping: {}", c.mapping());
	// 		return c;
	// 	},
	// 	None => panic!("Couldn't open any controller"),
	// };
	controller
}

//----------------------------------------------------------------

enum Command {
	Quit,
	Down(tetrs::Play),
	Up(tetrs::Play),
}
impl Command {
	fn from_event(e: Event) -> Option<Command> {
		use sdl2::event::Event::*;
		use sdl2::keyboard::Keycode::*;
		use sdl2::controller::Button;
		use tetrs::Play::*;
		match e {
			Quit { .. } => {
				Some(Command::Quit)
			},
			KeyDown { keycode, .. } => match keycode {
				Some(Left) => { Some(Command::Down(MoveLeft)) },
				Some(Right) => { Some(Command::Down(MoveRight)) },
				Some(Down) => { Some(Command::Down(SoftDrop)) },
				Some(Up) => { Some(Command::Down(RotateCW)) },
				Some(Space) => { Some(Command::Down(HardDrop)) },
				Some(LCtrl) => { Some(Command::Down(RotateCCW)) },
				_ => None,
			},
			KeyUp { keycode, .. } => match keycode {
				Some(Left) => { Some(Command::Up(MoveLeft)) },
				Some(Right) => { Some(Command::Up(MoveRight)) },
				Some(Down) => { Some(Command::Up(SoftDrop)) },
				Some(Up) => { Some(Command::Up(RotateCW)) },
				Some(Space) => { Some(Command::Up(HardDrop)) },
				Some(LCtrl) => { Some(Command::Up(RotateCCW)) },
				_ => None,
			},
			ControllerButtonDown { button, .. } => match button {
				Button::DPadLeft => { Some(Command::Down(MoveLeft)) },
				Button::DPadRight => { Some(Command::Down(MoveRight)) },
				Button::DPadDown => { Some(Command::Down(SoftDrop)) },
				Button::X => { Some(Command::Down(RotateCCW)) },
				Button::Y => { Some(Command::Down(HardDrop)) },
				Button::B => { Some(Command::Down(RotateCW)) },
				Button::A => { Some(Command::Down(HardDrop)) },
				_ => None,
			},
			ControllerButtonUp { button, .. } => match button {
				Button::DPadLeft => { Some(Command::Up(MoveLeft)) },
				Button::DPadRight => { Some(Command::Up(MoveRight)) },
				Button::DPadDown => { Some(Command::Up(SoftDrop)) },
				Button::X => { Some(Command::Up(RotateCCW)) },
				Button::Y => { Some(Command::Up(HardDrop)) },
				Button::B => { Some(Command::Up(RotateCW)) },
				Button::A => { Some(Command::Up(HardDrop)) },
				_ => None,
			},
			_e => {
				// println!("event: {:?}", _e);
				None
			},
		}
	}
}

//----------------------------------------------------------------

fn main() {
	// Initialize SDL2
	let sdl_context = sdl2::init().unwrap();
	let video = sdl_context.video().unwrap();

	let gcs = sdl_context.game_controller().unwrap();
	let _controller = open_controller(&gcs);

	let _image = sdl2::image::init(INIT_PNG).unwrap();

	// Create the window
	let window = video.window("Tetrs", 520, 600)
		.position_centered().opengl()
		.build().unwrap();

	let mut cg = {
		let renderer = window.renderer()
		.accelerated()
		.build().unwrap();

		let atlas = renderer.load_texture("assets/sprites.png").unwrap();
		let background = renderer.load_texture("assets/background.png").unwrap();

		let style = 3;
		let style_y = 22 * style + 1;
		let sprites = Sprites {
			pieces: [
				Rect::new(22 * 0 + 1, style_y, 20, 20),
				Rect::new(22 * 1 + 1, style_y, 20, 20),
				Rect::new(22 * 2 + 1, style_y, 20, 20),
				Rect::new(22 * 3 + 1, style_y, 20, 20),
				Rect::new(22 * 4 + 1, style_y, 20, 20),
				Rect::new(22 * 5 + 1, style_y, 20, 20),
				Rect::new(22 * 6 + 1, style_y, 20, 20),
				Rect::new(22 * 7 + 1, style_y, 20, 20),
			],
			ghost: Rect::new(22 * 8 + 1, style_y, 20, 20),
		};
		let map = Map {
			field_x: 160,
			field_y: 97,
		};

		Graphics {
			renderer: renderer,
			atlas: atlas,
			background: background,
			sprites: sprites,
			map: map,
		}
	};

	// Event pump
	let mut events = sdl_context.event_pump().unwrap();

	// Tetris game state
	let mut state = tetrs::State::new(10, 22);
	let mut bot = tetrs::PlayI { score: 0.0, play: Vec::new(), player: None };
	let mut play_i = 0;
	let mut bag = tetrs::OfficialBag::default();
	let speed = tetrs::Clock {
		gravity: 40,
		player: 8,
	};
	let mut timers = speed;
	let mut action = tetrs::Play::Idle;

	'quit: loop {
		if !state.is_game_over() && state.player().is_none() {
			use tetrs::Bag;
			let next_piece = bag.next(state.well()).unwrap();
			if !state.spawn(next_piece) {
				bot = tetrs::PlayI::play(&tetrs::Weights::default(), state.well(), *state.player().unwrap());
				play_i = 0;
			}
		}

		for e in events.poll_iter() {
			match Command::from_event(e) {
				Some(Command::Quit) => break 'quit,
				Some(Command::Down(play)) => {
					// println!("action={:?}", action);
					match play {
						tetrs::Play::RotateCW => {
							state.rotate_cw();
						},
						tetrs::Play::RotateCCW => {
							state.rotate_ccw();
						},
						tetrs::Play::HardDrop => {
							state.hard_drop();
						},
						tetrs::Play::MoveLeft | tetrs::Play::MoveRight | tetrs::Play::SoftDrop => {
							action = play;
						},
						_ => {},
					};
				},
				Some(Command::Up(play)) => {
					if play == tetrs::Play::MoveLeft || play == tetrs::Play::MoveRight || play == tetrs::Play::SoftDrop {
						action = tetrs::Play::Idle;
					}
				},
				None => {
				},
			}
		}
		// println!("{:?}", action);

		// Decrement the timers
		timers.gravity -= 1;
		if timers.gravity == 0 {
			timers.gravity = speed.gravity;
			// state.gravity();
		}
		if timers.player > 0 {
			timers.player -= 1;
		}
		else {
			match action {
				tetrs::Play::MoveLeft => {
					state.move_left();
					timers.player = speed.player;
				},
				tetrs::Play::MoveRight => {
					state.move_right();
					timers.player = speed.player;
				},
				tetrs::Play::SoftDrop => {
					state.soft_drop();
					timers.player = speed.player;
				},
				_ => {},
			}
		}

		if play_i < bot.play.len() {
			match bot.play[play_i] {
				tetrs::Play::Idle => (),
				tetrs::Play::MoveLeft => { state.move_left(); },
				tetrs::Play::MoveRight => { state.move_right(); },
				tetrs::Play::RotateCW => { state.rotate_cw(); },
				tetrs::Play::RotateCCW => { state.rotate_ccw(); },
				tetrs::Play::SoftDrop => { state.soft_drop(); },
				tetrs::Play::HardDrop => { state.hard_drop(); },
			}
			play_i += 1;
		}

		state.clear_lines(|_| ());

		draw(&mut cg, &state.scene());

		thread::sleep(Duration::from_millis(16));
	}
}
