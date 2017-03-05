/*!
*/

extern crate tetrs_core as tetrs;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

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

fn main() {
	// Initialize SDL2
	let sdl_context = sdl2::init().unwrap();
	let video = sdl_context.video().unwrap();

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
			let next_piece = tetrs::PlayI::worst(&tetrs::PlayW::new(), state.well());
			state.spawn(next_piece);
		}

		for e in events.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;
			match e {
				Quit { .. } => break 'quit,
				KeyDown { keycode, .. } => match keycode {
					Some(Left) => { state.move_left(); },
					Some(Right) => { state.move_right(); },
					Some(Down) => { state.soft_drop(); },
					Some(Up) => { state.hard_drop(); },
					Some(Space) => { state.rotate_cw(); },
					_ => (),
				},
				_ => (),
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
