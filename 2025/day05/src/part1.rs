use crate::parsing::parse_input;
use aoc_helper::range::merge;

pub fn process(_input: &str) -> usize {
    let (ranges, item) = parse_input(_input);
    let ranges = merge(ranges);
    item.into_iter().filter(|x| ranges.iter().any(|(a,b)| a<= x && x <=b) ).count()

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
        assert_eq!(process(fixture), 3);
    }

}
