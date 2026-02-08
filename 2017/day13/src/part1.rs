pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .skip(1)
        .map(|line| line.split_once(": ").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .map(|(a, b)| {
            let range = (b - 1) * 2;
            if a.is_multiple_of(range) { a * b } else { 0 }
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"0: 3
1: 2
4: 4
6: 4"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 24);
    }
}
