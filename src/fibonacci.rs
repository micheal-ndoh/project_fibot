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
