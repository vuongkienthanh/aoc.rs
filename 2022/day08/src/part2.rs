pub fn process(_input: &str) -> usize {
    let input: Vec<Vec<u8>> = _input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let rows = input.len();
    let cols = input[0].len();
    let mut ans = 0;
    for r in 0..rows {
        for c in 0..cols {
            let center = input[r][c];
            let left = (0..c)
                .rev()
                .enumerate()
                .find_map(|(i, c)| (input[r][c] >= center).then_some(i + 1))
                .unwrap_or(c);
            let right = (c + 1..cols)
                .enumerate()
                .find_map(|(i, c)| (input[r][c] >= center).then_some(i + 1))
                .unwrap_or(cols - c-1);
            let top = (0..r)
                .rev()
                .enumerate()
                .find_map(|(i, r)| (input[r][c] >= center).then_some(i + 1))
                .unwrap_or(r);
            let bottom = (r + 1..rows)
                .enumerate()
                .find_map(|(i, r)| (input[r][c] >= center).then_some(i + 1))
                .unwrap_or(rows - r -1);
            ans = ans.max(top * bottom * left * right);
        }
    }
    ans
}
