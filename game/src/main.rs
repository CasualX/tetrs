/*!
*/

extern crate tetrs;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::GameControllerSubsystem;
use sdl2::controller::GameController;

use std::time::Duration;
use std::thread;

fn color(piece: Option<tetrs::Piece>) -> Color {
	match piece {
		Some(tetrs::Piece::O) => Color::RGB(0, 0, 170),
		Some(tetrs::Piece::I) => Color::RGB(170, 0, 0),
		Some(tetrs::Piece::S) => Color::RGB(0, 170, 0),
		Some(tetrs::Piece::Z) => Color::RGB(0, 170, 170),
		Some(tetrs::Piece::L) => Color::RGB(170, 0, 170),
		Some(tetrs::Piece::J) => Color::RGB(170, 170, 0),
		Some(tetrs::Piece::T) => Color::RGB(170, 85, 0),
		None => Color::RGB(170, 170, 170),
	}
}

fn draw(renderer: &mut Renderer, scene: &tetrs::Scene) {
	for row in 0..scene.height() {
		let line = scene.line(row);
		for col in 0..scene.width() {
			let tile = line[col as usize];
			let rect = Rect::new(col * 16, row * 16, 16, 16);

			use tetrs::TileTy::*;
			match tile.tile_ty() {
				Field => {
					renderer.set_draw_color(color(tile.piece()));
					renderer.fill_rect(rect).unwrap();
				},
				Ghost => {
					renderer.set_draw_color(Color::RGB(50, 50, 50));
					renderer.fill_rect(rect).unwrap();
				},
				Player => {
					renderer.set_draw_color(color(tile.piece()));
					renderer.fill_rect(rect).unwrap();
				},
				Background => {
				},
			};
		}
	}
}

fn open_controller(gcs: &GameControllerSubsystem) -> GameController {
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

	match controller {
		Some(c) => {
			println!("Controller mapping: {}", c.mapping());
			return c;
		},
		None => panic!("Couldn't open any controller"),
	};
}

fn main() {
	// Initialize SDL2
	let sdl_context = sdl2::init().unwrap();
	let video = sdl_context.video().unwrap();

	let gcs = sdl_context.game_controller().unwrap();
	let _controller = open_controller(&gcs);

	// Create the window
	let window = video.window("Tetrs", 160, 352)
		.position_centered().opengl()
		.build().unwrap();

	let mut renderer = window.renderer()
		.accelerated()
		.build().unwrap();

	// Event pump
	let mut events = sdl_context.event_pump().unwrap();

	// Tetris game state
	let mut state = tetrs::State::new(10, 22);

    'quit: loop {
		if !state.is_game_over() && state.player().is_none() {
			let next_piece = tetrs::PlayI::worst_piece(&tetrs::Weights::new(), state.well());
			state.spawn(next_piece);
		}

		for e in events.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;
			use sdl2::controller::Button;
			match e {
				Quit { .. } => break 'quit,
				KeyDown { keycode, .. } => match keycode {
					Some(Left) => { state.move_left(); },
					Some(Right) => { state.move_right(); },
					Some(Down) => { state.soft_drop(); },
					Some(Up) => { state.rotate_cw(); },
					Some(Space) => { state.hard_drop(); },
					Some(LCtrl) => { state.rotate_ccw(); },
					_ => (),
				},
				ControllerButtonDown { button, .. } => match button {
					Button::DPadLeft => { state.move_left(); },
					Button::DPadRight => { state.move_right(); },
					Button::DPadDown => { state.soft_drop(); },
					Button::X => { state.rotate_ccw(); },
					Button::B => { state.rotate_cw(); },
					Button::A => { state.hard_drop(); },
					_ => (),
				},
				e => {
					println!("event: {:?}", e);
				},
			};
		}

		state.clear_lines(|_| ());

		// Render a fully black window
		renderer.set_draw_color(Color::RGB(0, 0, 0));
		renderer.clear();
		
		draw(&mut renderer, &state.scene());

		renderer.present();

		thread::sleep(Duration::from_millis(1));
    }
}
