use die::die;

use std::{thread, time};

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

	/// FIXME: Filter out 'userInput' variable
	/// If 'userInput' stores an integer -> Convert in float and assign in `n`
	/// if userInput stores float -> Assign in `n` 
	/// If userInput stores string -> Err2
	/// If userInput stores anything else -> Err256

	// Core
	while n <= 255.0 {
		println!("fibbonacci {}", fib(n));

		// Wait 500 ms after each output to avoid flodding the console
		thread::sleep(time::Duration::from_millis(500));
	}

	die!(0; "Success!");
}