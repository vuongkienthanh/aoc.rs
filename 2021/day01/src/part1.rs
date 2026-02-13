pub fn process(_input: &str) -> usize {
    let input: Vec<_> = _input
        .lines()
        .map(|x| x.parse::<u16>().unwrap())
        .collect();
    input.windows(2).filter(|v| v[1] > v[0]).count()
}
