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
        r#"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2"#
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
