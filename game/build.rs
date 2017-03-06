
use std::env;

fn main() {
	println!("cargo:rustc-link-lib=static=sdl2");
	println!("cargo:rustc-link-search=native={}/../sdl2/{}",
		env::var("CARGO_MANIFEST_DIR").unwrap(),
		env::var("TARGET").unwrap()
	);
}
