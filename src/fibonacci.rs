pub fn fibonacci(n: u32, max_value: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut fib_sequence = vec![0, 1];
    while let Some(&_last) = fib_sequence.last() {
        let next = fib_sequence[fib_sequence.len() - 1] + fib_sequence[fib_sequence.len() - 2];
        if next > max_value {
            break;
        }
        fib_sequence.push(next);
    }
    
    *fib_sequence.get(n as usize).unwrap_or(&0) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_small_numbers() {
        assert_eq!(fibonacci(0, 100), 0);
        assert_eq!(fibonacci(1, 100), 1);
        assert_eq!(fibonacci(2, 100), 1);
        assert_eq!(fibonacci(3, 100), 2);
    }

    #[test]
    fn test_fibonacci_large_numbers() {
        assert_eq!(fibonacci(10, 1000), 55);
        assert_eq!(fibonacci(20, 1000), 6765);
    }

    #[test]
    fn test_fibonacci_max_threshold() {
        assert_eq!(fibonacci(100, 100), 89); 
    }
}
