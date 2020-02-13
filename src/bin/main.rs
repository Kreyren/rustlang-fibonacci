use std::string::String;
use std::io;
use die::Die;
use std::convert::TryInto;

use kreybonacci::fibonacci;
use kreybonacci::fixme;

fn main() {
	#[allow(non_snake_case)] // Makes it more readable
	let mut userInput = String::new();

	// Process userInput
	fixme!("Output helpful warning for fixme_argument_parsed which requires contrib in die crate");
	io::stdin().read_line(&mut userInput)
		.die_code("Unexpected input 'fixme_argument_parsed' has been parsed", 2);
	let n: u128 = userInput.trim_end_matches("\n").parse()
		.die_code("Invalid argument 'fixme_argument' has been parsed", 2);

	// FIXME: Output based on userinput
	println!("{:?}", fibonacci().take(n.try_into().unwrap()));
}