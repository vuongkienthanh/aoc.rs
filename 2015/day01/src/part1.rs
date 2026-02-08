pub fn process(_input: &str) -> isize {
    _input.chars().fold(0, |acc, x| match x {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())",-1)]
    #[case("))(",-1)]
    #[case(")))",-3)]
    #[case(")())())",-3)]
    fn test_process(#[case] input: &str, #[case] expected: isize) {
        assert_eq!(process(input), expected);
    }
}
