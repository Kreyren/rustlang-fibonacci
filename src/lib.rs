use die::die;

// Fibonnaci sequence
pub fn fib(n: f64) -> f64 {
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