use fxhash::FxHashSet as Set;

pub fn process(_input: &str) -> usize {
    _input.lines().filter(|x| is_valid(x)).count()
}

fn is_valid(input:&str) -> bool {
    let mut seen = Set::default();
    for word in input.split_ascii_whitespace() {
        if !seen.insert(word) {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("aa bb cc dd ee", true)]
    #[case("aa bb cc dd aa", false)]
    #[case("aa bb cc dd aaa", true)]
    fn test_is_valid(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_valid(input), expected);
    }
}
