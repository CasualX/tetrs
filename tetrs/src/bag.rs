
use ::rand::{Rng, ThreadRng, thread_rng};

use ::{Piece, Well, Weights, PlayI};

/// The Random Generator.
pub trait Bag {
	/// Produce the next piece.
	fn next(&mut self, well: &Well) -> Option<Piece>;
	/// Let the player see the queued up pieces.
	fn peek(&self) -> &[Piece] {
		&[]
	}
}

/// Official Random Generator.
///
/// Source: http://tetris.wikia.com/wiki/Random_Generator
///
/// > The Random Generator generates a sequence of all seven tetrominoes permuted randomly as if they were drawn from a bag.
/// > Then it deals all seven tetrominoes to the piece sequence before generating another bag.
///
/// Because of the ability to peek ahead at the next piece, must keep track of the next seven tetrominoes as well.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OfficialBag<R: Rng> {
	rng: R,
	bag: [Piece; 14],
	pos: u8,
}
impl<R: Rng> OfficialBag<R> {
	pub fn with_rng(rng: R) -> OfficialBag<R> {
		OfficialBag {
			rng: rng,
			bag: [Piece::O, Piece::I, Piece::S, Piece::Z, Piece::L, Piece::J, Piece::T,
			      Piece::O, Piece::I, Piece::S, Piece::Z, Piece::L, Piece::J, Piece::T],
			pos: 255,
		}
	}
}
impl Default for OfficialBag<ThreadRng> {
	fn default() -> OfficialBag<ThreadRng> {
		OfficialBag::with_rng(thread_rng())
	}
}
impl<R: Rng> Bag for OfficialBag<R> {
	fn next(&mut self, _well: &Well) -> Option<Piece> {
		let (left, right) = self.bag.split_at_mut(7);
		if self.pos >= 14 {
			self.rng.shuffle(right);
		}
		if self.pos >= 7 {
			left.copy_from_slice(right);
			self.rng.shuffle(right);
			self.pos = 0;
		}
		let next_piece = left[self.pos as usize];
		self.pos += 1;
		Some(next_piece)
	}
	fn peek(&self) -> &[Piece] {
		let pos = self.pos as usize;
		&self.bag[pos..pos + 7]
	}
}

/// Pieces bag generously giving the best pieces.
#[derive(Clone, Debug, Default)]
pub struct BestBag {
	weights: Weights,
}
impl BestBag {
	pub fn new(weights: Weights) -> BestBag {
		BestBag {
			weights: weights,
		}
	}
}
impl Bag for BestBag {
	fn next(&mut self, well: &Well) -> Option<Piece> {
		let next_piece = PlayI::best_piece(&self.weights, well);
		Some(next_piece)
	}
}

/// Pieces bag coldly giving the worst pieces.
#[derive(Clone, Debug, Default)]
pub struct WorstBag {
	weights: Weights,
}
impl WorstBag {
	pub fn new(weights: Weights) -> WorstBag {
		WorstBag {
			weights: weights,
		}
	}
}
impl Bag for WorstBag {
	fn next(&mut self, well: &Well) -> Option<Piece> {
		let next_piece = PlayI::worst_piece(&self.weights, well);
		Some(next_piece)
	}
}
