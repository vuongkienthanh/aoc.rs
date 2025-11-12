fn code_representation(input: &str) -> usize {
    input.len()
}
fn decode_representation(input: &str) -> usize {
    let mut len = input.len();
    for c in input.chars() {
        match c {
            '\"' => len += 1,
            '\\' => len += 1,
            _ => (),
        }
    }
    len + 2
}
pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| decode_representation(line) - code_representation(line))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("\"\"", 6)]
    #[case("\"abc\"", 9)]
    #[case("\"aaa\\\"aaa\"", 16)]
    #[case("\"\\x27\"", 11)]
    fn test_decode_representation(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(decode_representation(input), expected);
    }
}
