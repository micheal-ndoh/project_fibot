pub fn fibonacci(n: u32, max_value: u32) -> Vec<u32> {
    let mut fib_sequence = vec![0, 1];
    while let Some(&_last) = fib_sequence.last() {
        let next = fib_sequence[fib_sequence.len() - 1] + fib_sequence[fib_sequence.len() - 2];
        if next > max_value {
            println!("Reached maximum threshold value: {}", max_value);
            break;
        }
        fib_sequence.push(next);
    }
    fib_sequence.into_iter().take(n as usize).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_with_max_value() {
        let max_value = 100;
        let result = fibonacci(10, max_value);
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fibonacci_exceeds_max_value() {
        let max_value = 20;
        let result = fibonacci(10, max_value);
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13];
        assert_eq!(result, expected);
    }
}