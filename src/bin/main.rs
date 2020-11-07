use my_fibonacci::fibonacci;
use std::{io::stdin, io::stdout, io::Write};

fn main() {
    let mut n: String = String::new();
    print!("Index: ");

    stdout().flush().unwrap();
    stdin()
        .read_line(&mut n)
        .expect("Failed to read from stdin!");

    match n.trim().parse::<u64>() {
        Ok(x) => println!("Index: {:1} Fibonacci number: {:1}", x, fibonacci(x)),
        Err(..) => println!("Integer Expected! Found: '{}'.", n),
    };
}
