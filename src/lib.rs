pub fn fibonacci(n: u64) -> u64 {
    (0..n).fold((0, 1), |(a, b), _| (b, b + a)).0
}

#[cfg(test)]
mod test {
    /* use every fn from above */
    use super::*;

    #[test]
    fn fibonacci_1st() {
        assert_eq!(1, fibonacci(1));
    }

    #[test]
    fn fibonacci_2nd() {
        assert_eq!(1, fibonacci(2));
    }

    #[test]
    fn fibonacci_8th() {
        assert_eq!(21, fibonacci(8));
    }

    #[test]
    fn fibonacci_50th() {
        assert_eq!(12586269025, fibonacci(50));
    }

    #[test]
    fn fibonacci_92nd() { assert_eq!(7540113804746346429, fibonacci(92)); }

    #[test]
    fn fibonacci_1000th() {
        assert_eq!(817770325994397771, fibonacci(1000));
    }
}
