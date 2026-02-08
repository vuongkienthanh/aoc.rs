pub fn process(_input: &str) -> usize {
    let (mut left, mut right) = _input
        .lines()
        .map(|s| {
            let mut w = s.split_ascii_whitespace();
            (
                w.next().unwrap().parse::<usize>().unwrap(),
                w.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold(
            (Vec::<usize>::new(), Vec::<usize>::new()),
            |mut acc, ele| {
                acc.0.push(ele.0);
                acc.1.push(ele.1);
                acc
            },
        );
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(process(input), 11);
    }
}
