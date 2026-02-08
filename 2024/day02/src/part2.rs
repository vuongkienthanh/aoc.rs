pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|ele| ele.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|line| {
            (0..line.len()).any(|dampen| {
                let shrunken = line
                    .iter()
                    .take(dampen)
                    .chain(line.iter().skip(dampen + 1))
                    .collect::<Vec<_>>();
                shrunken.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
                    || shrunken.windows(2).all(|w| w[1] < w[0] && w[0] - w[1] <= 3)
            })
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(process(input), 4);
    }
}
