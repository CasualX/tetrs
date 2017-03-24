extern crate rand;
extern crate tetrs;

use rand::{thread_rng, Rng};

const NUM_GAMES: usize = 100;
const MAX_MOVES: usize = 2000;
const POP_SIZE: usize = 100;
const MAX_ITERS: usize = 200;

fn main() {
	let mut rng = thread_rng();

	// Learning from entropy :)
	let mut best_weights: tetrs::Weights = rng.gen();
	let mut best_score = fitness(&best_weights);
	let mut iterations = 0;

	loop {
		let weights: tetrs::Weights = rng.gen();
		let score = fitness(&weights);
		if score < best_score {
			best_weights = weights;
			best_score = score;
			iterations = 0;
			// println!("{:#?}", weights);
		}
		iterations += 1;
		if iterations > MAX_ITERS {
			break;
		}
	}
	println!("{:#?}", best_weights);
}

fn fitness(weights: &tetrs::Weights) -> i32 {
	let mut fitness = 0;
	for _ in 0..NUM_GAMES {
		fitness += play_game(weights);
	}
	fitness
}

fn play_game(weights: &tetrs::Weights) -> i32 {
	let mut state = tetrs::State::new(10, 11); // Reduce number of rows for speedup
	let mut bag = tetrs::OfficialBag::default();
	let mut score = 0;
	let mut moves = 0;
	loop {
		// Spawn a new player
		use tetrs::Bag;
		let next_piece = bag.next(state.well()).unwrap();
		state.spawn(next_piece);

		// Let the AI play a piece
		let &player = state.player().unwrap();
		let bot = tetrs::PlayI::play(&weights, state.well(), player);

		// No need to actually play the moves, just teleport the player
		if let Some(player) = bot.player {
			state.set_player(player);
			state.lock();
		}
		else {
			// Game over, didn't find a valid move that wouldn't make us lose
			break;
		}

		// Clear the lines
		state.clear_lines(|_| score += 1);

		// Break out if the AI is too good :)
		moves += 1;
		if moves >= MAX_MOVES {
			println!("breakout!");
			break;
		}
	}
	score
}
