extern crate tetrs;
extern crate rand;
use rand::Rng;

use ::std::io::prelude::*;

// FIXME! Little hack to clear the screen :)
extern "C" { fn system(s: *const u8); }
fn clear_screen() { unsafe {
	system("@clear||cls\0".as_ptr());
}}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Input {
	None,
	Left,
	Right,
	RotateCW,
	RotateCCW,
	SoftDrop,
	HardDrop,
	Gravity,
	Quit,
	Help,
	Invalid,
}
fn input() -> Input {
	print!(">>> ");
	std::io::stdout().flush().unwrap();
	let mut action = String::new();
	std::io::stdin().read_line(&mut action).unwrap();
	match &*action.trim().to_uppercase() {
		"" => Input::None,
		"L" | "LEFT" => Input::Left,
		"R" | "RIGHT" => Input::Right,
		"CW" | "RR" | "ROT" => Input::RotateCW,
		"CCW" | "RL" => Input::RotateCCW,
		"D" | "DOWN" | "SOFT" | "SOFT DROP" => Input::SoftDrop,
		"DROP" | "HARD DROP" => Input::HardDrop,
		"G" | "GRAVITY" => Input::Gravity,
		"Q" | "QUIT" | "QUTI" => Input::Quit,
		"H" | "HELP" => Input::Help,
		_ => Input::Invalid,
	}
}

fn bot(bot: &tetrs::PlayerBot, state: &mut tetrs::State) -> bool {
	bot.play(state);
	true
}

fn main() {
	clear_screen();

	let mut state = tetrs::State::new(10, 12);
	let mut next_piece = tetrs::Piece::J;
	state.spawn(tetrs::Piece::I);
	let player_bot = tetrs::PlayerBot::new();
	let mut rng = rand::thread_rng();

	loop {
		println!("{}", state);

		match input() {
			Input::None => bot(&player_bot, &mut state),
			Input::Quit => break,
			Input::Left => state.move_left(),
			Input::Right => state.move_right(),
			Input::RotateCW => state.rotate_cw(),
			Input::RotateCCW => state.rotate_ccw(),
			Input::SoftDrop => state.soft_drop(),
			Input::HardDrop => state.hard_drop(),
			Input::Gravity => state.gravity(),
			_ => true,
		};

		// Spawn a new piece as needed.
		if state.player().is_none() {
			if state.spawn(next_piece) {
				println!("Game Over!");
				break;
			}
			let r: u8 = rng.gen();
			next_piece = unsafe { std::mem::transmute(r % 7) };
		}

		state.clear_lines(|_| ());
		clear_screen();
	}

	println!("Thanks for playing!");
}
