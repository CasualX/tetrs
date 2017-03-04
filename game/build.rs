
fn main() {
	println!("cargo:rustc-link-lib=static=sdl2");
	println!("cargo:rustc-link-search=native=C:\\Users\\Dries\\Programs\\SDL2-2.0.5\\lib\\x64");
}
