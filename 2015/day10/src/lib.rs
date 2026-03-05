pub mod part1;
pub mod part2;

pub fn next(input: String) -> String {
    let mut ret = String::new();
    let mut i = input.chars();
    let mut current_char = i.next().unwrap();
    let mut current_count = 1;

    for c in i {
        if c == current_char {
            current_count += 1;
        } else {
            ret.push_str(format!("{current_count}{current_char}").as_str());
            current_char = c;
            current_count = 1;
        }
    }
    ret.push_str(format!("{current_count}{current_char}").as_str());
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
