pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let mut min = usize::MAX;
            let mut max = usize::MIN;
            for c in line.split_ascii_whitespace() {
                let c = c.parse::<usize>().unwrap();
                if c < min {
                    min = c;
                }
                if c > max {
                    max = c;
                }
            }
            max - min
        })
        .sum()
}
