use fxhash::FxHashSet as Set;

pub fn process(_input: &str) -> usize {
    _input.lines().filter(|x| is_valid(x)).count()
}

fn is_valid(input: &str) -> bool {
    let mut seen = Set::default();
    for word in input.split_ascii_whitespace() {
        if !seen.insert(word) {
            return false;
        }
    }
    true
}

fn word_to_arr(input:&str) -> [usize; 26] {

}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("abcde fghij", true)]
    #[case("abcde xyz ecdab", false)]
    #[case("a ab abc abd abf abj", true)]
    #[case("iiii oiii ooii oooi oooo", true)]
    #[case("oiii ioii iioi iiio", false)]
    fn test_is_valid(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_valid(input), expected);
    }
}
