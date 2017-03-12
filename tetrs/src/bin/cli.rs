extern crate tetrs;

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
	let weights = tetrs::Weights::default();
	let bot = tetrs::PlayI::play(&weights, state.well(), *state.player().unwrap());
	if bot.play.len() == 0 {
		state.hard_drop();
		return false;
	}
	let mut result = true;
	for play in bot.play {
		use tetrs::Play;
		result &= match play {
			Play::MoveLeft => state.move_left(),
			Play::MoveRight => state.move_right(),
			Play::RotateCW => state.rotate_cw(),
			Play::RotateCCW => state.rotate_ccw(),
			Play::SoftDrop => state.soft_drop(),
			Play::HardDrop => state.hard_drop(),
			Play::Idle => true,
		};
		if !result {
			break;
		}
	}
	result
}

static TILESET: [char; 32] = [
	'O', 'I', 'S', 'Z', 'L', 'J', 'T', 'x',
	'_', '_', '_', '_', '_', '_', '_', 'x',
	'O', 'I', 'S', 'Z', 'L', 'J', 'T', '□',
	'.', '_', ' ', 'x', 'x', 'x', 'x', 'x',
];

fn draw(scene: &tetrs::Scene) {
	for row in 0..scene.height() {
		print!("|");
		let line = scene.line(row);
		for &tile in line {
			let tile: u8 = tile.into();
			let c = TILESET[(tile >> 3) as usize];
			print!("{}", c);
		}
		print!("|\n");
	}
	print!("+");
	for _ in 0..scene.width() {
		print!("-");
	}
	print!("+\n");
}

const WELCOME_MESSAGE: &'static str = "
Welcome to Adventure Tetrs!
After the playing field is shown, you will be asked for input.

>>> A, Q, LEFT
Move the piece to the left.
>>> D, RIGHT
Move the piece to the right.
>>> CW, RR, ROT
Rotate the piece clockwise.
>>> CCW, RL
Rotate the piece counter-clockwise.
>>> S, DOWN, SOFT, SOFT DROP
Soft drop, move the piece down once.
>>> W, Z, DROP, HARD DROP
Hard drop, drops the piece down and locks into place.
>>> G, GRAVITY
Apply gravity, same as a soft drop.
>>> QUIT, QUTI
Quit the game.
>>> H, HELP
Print this help message.

";

fn main() {
	clear_screen();
	
	println!("{}", WELCOME_MESSAGE);

	use tetrs::Bag;

	let mut state = tetrs::State::new(10, 22);
	let mut bag = tetrs::OfficialBag::default();
	let mut next_piece = bag.next(state.well()).unwrap();
	state.spawn(next_piece);

	loop {
		draw(&state.scene());

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
			next_piece = bag.next(state.well()).unwrap();
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
