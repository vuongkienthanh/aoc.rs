use crate::parsing::parse_input;
use aoc_helper::range::merge;

pub fn process(_input: &str) -> usize {
    let (ranges, _) = parse_input(_input);
    let ranges = merge(ranges);

    ranges.into_iter().map(|(a, b)| b - a + 1).sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 14);
    }
}
