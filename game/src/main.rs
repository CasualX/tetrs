/*!
*/

extern crate tetrs_core as tetrs;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

// mod events;

fn draw(renderer: &mut Renderer, state: &tetrs::State) {
	// Create a new well with all the blocks in it
	let mut well = state.well().clone();
	if let Some(player) = state.player() {
		well.etch(player);
	}

	renderer.set_draw_color(Color::RGB(200, 200, 200));

	// Draw the pieces
	well.describe(|pt| {
		let x = pt.x * 16;
		let y = (well.height() - pt.y - 1) * 16;

		let rc = Rect::new(x, y, 16, 16);
		renderer.fill_rect(rc).unwrap();
	});
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
		
		draw(&mut renderer, &state);

		renderer.present();
    }
}
