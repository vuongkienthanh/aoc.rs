use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    _input
        .chars()
        .fold(
            (HashSet::from([(0, 0)]), (0, 0)),
            |(mut acc, mut coord), c| {
                match c {
                    '^' => coord.1 += 1,
                    'v' => coord.1 -= 1,
                    '<' => coord.0 -= 1,
                    '>' => coord.0 += 1,
                    _ => (),
                };

                acc.insert(coord);
                (acc, coord)
            },
        )
        .0
        .len()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
