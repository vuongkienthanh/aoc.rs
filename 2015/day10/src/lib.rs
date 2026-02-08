pub mod part1;
pub mod part2;

pub fn next(input: String) -> String {
    assert!(!input.is_empty());

    let mut ret = String::new();
    let mut current_char = input.chars().next().unwrap();
    let mut current_count = 1;

    for c in input.chars().skip(1) {
        if c == current_char {
            current_count += 1;
        } else {
            ret.push(char::from_digit(current_count, 10).expect("should be a digit"));
            ret.push(current_char);
            current_char = c;
            current_count = 1;
        }
    }
    ret.push(char::from_digit(current_count, 10).expect("should be a digit"));
    ret.push(current_char);
    ret
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("1".to_string(), "11".to_string())]
    #[case("11".to_string(), "21".to_string())]
    #[case("21".to_string(), "1211".to_string())]
    #[case("1211".to_string(), "111221".to_string())]
    #[case("111221".to_string(), "312211".to_string())]
    fn test_next(#[case] input: String, #[case] expected: String) {
        assert_eq!(next(input), expected);
    }
}
