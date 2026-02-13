pub fn process(_input: &str) -> usize {
    let input: Vec<_> = _input.lines().map(|x| x.parse::<u16>().unwrap()).collect();
    let mut sum = 0;
    let mut count = 0;
    for v in input.windows(3) {
        let new_sum: u16 = v.iter().sum();
        if new_sum > sum {
            count += 1;
        }
        sum = new_sum;
    }
    count - 1
}
