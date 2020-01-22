use std::io;

fn main() {
	let mut x = String::new();
	let mut x = String::new();

	loop {
		let z = x + y;
		let x = y;
		let y = z;

		println!("{}", x);
		if x >= 255 {
			break
		}
	}
}
