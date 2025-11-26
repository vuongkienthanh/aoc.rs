pub fn process(_input: &str) -> usize {
    println!("Josephus problem");
    let input = _input.parse::<usize>().unwrap();
    // usize::bin_width is unstable
    let bin_width = format!("{:b}", input).len();
    (input << 1) ^ (1 << bin_width) | 1
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
