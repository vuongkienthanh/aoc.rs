pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let mut line: Vec<usize> = line
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            line.sort_unstable_by(|a, b| b.cmp(a));
            let mut ans = None;
            'l: for a in 0..line.len() - 1 {
                for b in a + 1..line.len() {
                    if line[a].is_multiple_of(line[b]) {
                        ans = Some(line[a] / line[b]);
                        break 'l;
                    }
                }
            }
            ans.unwrap()
        })
        .sum()
}
