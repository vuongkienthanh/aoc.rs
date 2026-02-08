pub fn process(_input: &str) -> usize {
    let mut sum = 0;
    let half = _input.len() / 2;
    let v: Vec<char> = _input.chars().collect();
    for i in 0.._input.len() {
        let j = (i + half) % _input.len();
        if v[i] == v[j] {
            sum += v[i].to_digit(10).unwrap() as usize;
        }
    }
    sum
}
