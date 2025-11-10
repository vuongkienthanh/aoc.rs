pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let dimensions = line
                .splitn(3, 'x')
                .map(|n| n.parse::<usize>().expect("a number"))
                .collect::<Vec<usize>>();
            let a1 = dimensions[0] * dimensions[1];
            let a2 = dimensions[1] * dimensions[2];
            let a3 = dimensions[0] * dimensions[2];
            let min_a = [a1, a2, a3].into_iter().min().unwrap();
            2 * (a1 + a2 + a3) + min_a
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
