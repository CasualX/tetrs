extern crate tetrs_core as tetrs;
extern crate rand;

use rand::Rng;
use std::io::prelude::*;

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
		"A" | "Q" | "LEFT" => Input::Left,
		"D" | "RIGHT" => Input::Right,
		"CW" | "RR" | "ROT" => Input::RotateCW,
		"CCW" | "RL" => Input::RotateCCW,
		"S" | "DOWN" | "SOFT" | "SOFT DROP" => Input::SoftDrop,
		"W" | "Z" | "DROP" | "HARD DROP" => Input::HardDrop,
		"G" | "GRAVITY" => Input::Gravity,
		"QUIT" | "QUTI" => Input::Quit,
		"H" | "HELP" => Input::Help,
		_ => Input::Invalid,
	}
}

fn bot(state: &mut tetrs::State) -> bool {
	let weights = tetrs::PlayW::new();
	let bot = tetrs::PlayI::best(&weights, state.well(), state.player().unwrap().piece);
	loop {
		let success = match bot.play(state) {
			tetrs::PlayM::Idle => return true,
			tetrs::PlayM::MoveLeft => state.move_left(),
			tetrs::PlayM::MoveRight => state.move_right(),
			tetrs::PlayM::RotateCW => state.rotate_cw(),
			tetrs::PlayM::RotateCCW => state.rotate_ccw(),
			tetrs::PlayM::SoftDrop => state.soft_drop(),
			tetrs::PlayM::HardDrop => state.hard_drop(),
		};
		if !success {
			return state.hard_drop();
		}
	}
}

const HATETRIS: bool = true;

fn main() {
	clear_screen();

	let mut state = tetrs::State::new(10, 22);
	let mut next_piece = if HATETRIS { tetrs::Piece::S } else { tetrs::Piece::L };
	state.spawn(next_piece);
	let mut rng = rand::thread_rng();

	loop {
		println!("{}", state);

		// Check for pieces in the spawning area
		if state.is_game_over() {
			println!("Game Over!");
			break;
		}

		match input() {
			Input::None => bot(&mut state),
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

		// Spawn a new piece as needed
		if state.player().is_none() {
			next_piece = if HATETRIS {
				tetrs::PlayI::worst(&tetrs::PlayW::new(), state.well())
			}
			else {
				let r: u8 = rng.gen();
				unsafe { std::mem::transmute(r % 7) }
			};
			if state.spawn(next_piece) {
				println!("Game Over!");
				break;
			}
		}

		state.clear_lines(|_| ());
		clear_screen();
	}

	println!("Thanks for playing!");
}
