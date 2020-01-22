use die::die;

use std::{thread, time};

fn fib(n: f64) -> f64 {
	if n <= 1.0 {
		return n;
	} else if n > 1.0 {
		return fib(n-1.0) + fib(n-2.0);
	} else {
		die!(256; "Unexpected happend while processing number '{}'", n);
	}
}

fn main() {
	// Input - FIXME: Implement grabbing from cli
	let n = 9.0;

	// Core
	loop {
		println!("{}", fib(n));

		// Wait 500 ms after each output to avoid flodding the console
		thread::sleep(time::Duration::from_millis(500));
	}	
}