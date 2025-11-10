use std::collections::HashSet;
pub fn process(_input: &str) -> usize {
    _input
        .chars()
        .zip([0, 1].into_iter().cycle())
        .fold(
            (HashSet::from([(0, 0)]), [(0, 0), (0, 0)]),
            |(mut acc, mut coords), (c, i)| {
                match c {
                    '^' => coords[i].1 += 1,
                    'v' => coords[i].1 -= 1,
                    '<' => coords[i].0 -= 1,
                    '>' => coords[i].0 += 1,
                    _ => (),
                };

                acc.insert(coords[i]);
                (acc, coords)
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
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
