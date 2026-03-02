pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let line: Vec<_> = line.bytes().collect();
            let mut ans = 0;
            for (i, c) in line.windows(4).enumerate() {
                let seen: u32 = 1 << (c[0] - b'a')
                    | 1 << (c[1] - b'a')
                    | 1 << (c[2] - b'a')
                    | 1 << (c[3] - b'a');
                if seen.count_ones() == 4 {
                    ans = i + 4;
                    break;
                }
            }
            ans
        })
        .sum()
}
