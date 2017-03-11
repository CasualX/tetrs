extern crate tetrs;

#[inline(never)]
fn extern_tag(_: &str) {}

fn main() {
	let well = tetrs::Well::from_data(10, &[
		0b0000110000,
		0b0111111001,
		0b0110111111,
		0b1111111111,
		0b1110111111,
		0b1111111111,
	]);
	extern_tag("extern_tag1");
	well.test_player(tetrs::Player::new(tetrs::Piece::O, tetrs::Rot::Zero, tetrs::Point::new(2, 3)));
	extern_tag("extern_tag2");
}
