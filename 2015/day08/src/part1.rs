fn code_representation(input: &str) -> usize {
    input.len()
}

fn string_representation(input: &str) -> usize {
    let mut len = input.len();
    len -= 2;
    let mut i = input.chars();
    while let Some(c) = i.next() {
        if c == '\\' {
            let nc = i.next().unwrap();
            if nc == 'x' {
                len -= 3;
            } else {
                len -= 1;
            }
        }
    }
    len
}

pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| code_representation(line) - string_representation(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("\"\"", 2)]
    #[case("\"abc\"", 5)]
    #[case("\"aaa\\\"aaa\"", 10)]
    #[case("\"\\x27\"", 6)]
    fn test_code_representation(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(code_representation(input), expected);
    }
    #[rstest]
    #[case("\"\"", 0)]
    #[case("\"abc\"", 3)]
    #[case("\"aaa\\\"aaa\"", 7)]
    #[case("\"\\x27\"", 1)]
    fn test_string_representation(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(string_representation(input), expected);
    }
}
