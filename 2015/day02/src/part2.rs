pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let mut dimensions = line
                .splitn(3, 'x')
                .map(|n| n.parse::<usize>().expect("a number"))
                .collect::<Vec<usize>>();
            dimensions.sort_unstable();

            let rbox = (dimensions[0] + dimensions[1]) * 2;
            let bow = dimensions.into_iter().product::<usize>();
            rbox + bow
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
