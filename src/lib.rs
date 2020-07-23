use die::die;

// Fibonnaci sequence
pub fn fib(n: u64) -> u64 {
	if n == 0 {
		return 0;
	} else if n == 1 {
		return 1;
	} else if n > 1 {
		return fib(n-1) + fib(n-2);
	} else {
		die!(256; "Unexpected happend while processing number '{}'", n);
	}
}
