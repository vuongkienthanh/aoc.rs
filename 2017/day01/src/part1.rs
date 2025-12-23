pub fn process(_input: &str) -> usize {
    let mut sum = 0;
    let mut i = _input.chars().chain(_input.chars().next()).peekable();
    for _ in 0.._input.len() {
        let a = i.next().unwrap();
        let b = i.peek().unwrap();
        if a == *b {
            sum += a.to_digit(10).unwrap() as usize;
        }
    }
    sum
}
