pub fn process(_input: &str) -> usize {
    for i in 0..usize::MAX {
        let inp = format!("{}{}", _input, i);
        let hex = format!("{:x}", md5::compute(inp.as_bytes()));
        if hex.starts_with("00000") {
            return i;
        }
    }
    panic!("should have an answer")
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
