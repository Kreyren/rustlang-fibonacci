use std::io;
use std::string::String;
use die::Die;

use fibonnaci_kreyren::fib;

macro_rules! fixme {
	($msg:expr) => ( println!("FIXME: {}", $msg);)
}

fn main() {
	#[allow(non_snake_case)] // Makes it more readable
	let mut userInput = String::new();

	// Process userInput
	io::stdin().read_line(&mut userInput)
		.die_code("Unexpected input 'fixme_input_value' has been parsed", 256);
	let n: f64 = userInput.trim_end_matches("\n").parse()
		.die_code("Invalid argument 'fixme_argument' has been parsed", 2);

	// FIXME: Optimize better for numbers greater then 41
	if n > 41.0 {
		fixme!("Function 'fib' takes too long to process numbers greater then 41");
	}

	// Output
	println!("{}", fib(n));
}