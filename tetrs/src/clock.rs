/*!
Game timers.
*/

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Clock {
	pub gravity: i32,
	pub pl_move: i32,
	pub pl_rotate: i32,
}
