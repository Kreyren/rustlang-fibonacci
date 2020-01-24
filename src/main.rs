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
	#[allow(non_snake_case)]
	let mut userInput = String::new();

	// Process userInput
	io::stdin().read_line(&mut userInput).die_code("Unexpected input 'fixme_input_value' has been parsed", 256);
	let n: f64 = userInput.trim_end_matches("\n").parse().die_code("Invalid argument 'fixme_argument' has been parsed", 2);

	// Output
	println!("{}", fib(n));
}