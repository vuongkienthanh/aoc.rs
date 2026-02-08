pub mod part1;
pub mod part2;

fn parse(input: &str) -> (usize, Vec<u32>) {
    let line_len = input.lines().nth(0).unwrap().len();
    let v = input
        .lines()
        .map(|line| {
            line.bytes().enumerate().fold(0, |acc, (i, c)| match c {
                b'#' => acc | 1 << i,
                _ => acc,
            })
        })
        .collect();

    (line_len, v)
}

pub fn tree(input: &[u32], line_len: usize, right: usize, down: usize) -> usize {
    let mut col = right;
    let mut ans = 0;
    for row in (down..input.len()).step_by(down) {
        if input[row] & (1 << col) == (1 << col) {
            ans += 1;
        }
        col = (col + right) % line_len;
    }
    ans
}
