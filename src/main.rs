use die::Die;
use die::die;
use std::io;
use std::string::String;

// Fibonnaci sequence
fn fib(n: f64) -> f64 {
	if n == 0.0 {
		return 0.0;
	} else if n == 1.0 {
		return 1.0;
	} else if n > 1.0 {
		return fib(n-1.0) + fib(n-2.0);
	} else {
		die!(256; "Unexpected happend while processing number '{}'", n);
	}
}

fn main() {
	// Input - FIXME: Implement grabbing from cli
	let mut userInput = String::new();

	// FIXME: Filter out 'userInput' variable
	// If userInput stores anything else -> Err256 

	io::stdin().read_line(&mut userInput).die_code("bad input", 256);
	let n: f64 = userInput.trim_end_matches("\n").parse().die_code("not a float", 2);
	println!("{}", fib(n));

	die!(0);
}