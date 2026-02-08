pub fn process(_input: &str) -> usize {
    let mut current_floor = 0;
    for (i, c) in _input.char_indices() {
        match c {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => (),
        };
        if current_floor == -1 {
            return i + 1;
        }
    }
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
