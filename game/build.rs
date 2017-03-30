
use std::env;

fn main() {
	// SDL2 lib
	println!("cargo:rustc-link-lib=static=sdl2");
	println!("cargo:rustc-link-search=native={}/../sdl2/{}",
		env::var("CARGO_MANIFEST_DIR").unwrap(),
		env::var("TARGET").unwrap()
	);

	// SDL2 image lib
	println!("cargo:rustc-link-lib=static=sdl2_image");
	println!("cargo:rustc-link-search=native={}/../sdl2_image/{}",
		env::var("CARGO_MANIFEST_DIR").unwrap(),
		env::var("TARGET").unwrap());
}
