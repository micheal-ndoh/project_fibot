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
            continue;
        }
        fib_sequence.push(next);
    }
    
    *fib_sequence.get(n as usize).unwrap_or(&0) 
}
