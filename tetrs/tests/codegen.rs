extern crate tetrs;

#[inline(never)]
fn extern_tag(_: &str) {}

#[inline(never)]
fn black_box<T>(_: T) {}

#[test]
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
	let player = tetrs::Player::new(tetrs::Piece::O, tetrs::Rot::Zero, tetrs::Point::new(2, 3));
	black_box(well.test(player.sprite(), player.pt));
	black_box(well.count_holes());
	extern_tag("extern_tag2");
}
