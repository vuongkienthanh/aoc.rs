use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());
    println!("{input:?}");

    todo!("part1")
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
    fn test_process_1(fixture: &str) {
        assert_eq!(process(fixture), 0);
    }

    #[rstest]
    #[case("", 0)]
    fn test_process_2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
