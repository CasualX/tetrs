
use ::sdl2::controller;
use ::sdl2::keyboard;

pub enum Command {
	Quit,
	ChangeTo_WorstBag,
	ChangeTo_BestBag,
	ChangeTo_OfficialBag,
	MoveLeft_Down,
	MoveLeft_Up,
	MoveRight_Down,
	MoveRight_Up,
	SoftDrop_Down,
	SoftDrop_Up,
	Rotate_CW,
	Rotate_CCW,
	HardDrop,
}
