use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (map, units) = parse_input(_input);
    println!("{map:?}");
    println!("{units:?}");

    todo!("part1");
    // panic!("should have an answer")
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#""#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 0);
    }

    #[rstest]
    #[case("", 0)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
