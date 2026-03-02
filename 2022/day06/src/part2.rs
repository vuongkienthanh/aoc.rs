pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let line: Vec<_> = line.bytes().collect();
            let mut ans = 0;
            for (i, c) in line.windows(14).enumerate() {
                let seen: u32 = c
                    .iter()
                    .map(|x| 1 << (x - b'a'))
                    .reduce(|a, b| a | b)
                    .unwrap();
                if seen.count_ones() == 14 {
                    ans = i + 14;
                    break;
                }
            }
            ans
        })
        .sum()
}
