pub fn process(_input: &str) -> usize {
    println!("Josephus problem");
    let mut input = _input.parse::<usize>().unwrap();
    let bin = format!("{:b}", input);
    let bin_len = bin.len();
    (input << 1) ^ (1 << bin_len) | 1
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("5", 3)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
