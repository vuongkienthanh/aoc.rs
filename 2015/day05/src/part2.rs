use micromap::{Map, Set};

pub fn is_nice(s: &str) -> bool {
    let origin = s.chars().collect::<Vec<_>>();
    let mut seen: Map<&[char], Set<usize, 16>, 15> = Map::new();
    origin.windows(2).enumerate().any(|(i, v)| {
        let k = seen.entry(v).or_default();
        k.insert(i) && k.insert(i + 1) && k.len() == 4
    }) && ((0..origin.len() - 2).any(|i| origin[i] == origin[i + 2]))
}

pub fn process(_input: &str) -> usize {
    _input.lines().filter(|x| is_nice(x)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("xxyxx", true)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn test_process(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_nice(input), expected);
    }
}
