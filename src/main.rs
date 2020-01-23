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
	let n = 200.0;

	// Core
	while n <= 255.0 {
		println!("fibbonacci {}", fib(n));

		// Wait 500 ms after each output to avoid flodding the console
		thread::sleep(time::Duration::from_millis(500));
	}

	die!(0; "Success!");
}