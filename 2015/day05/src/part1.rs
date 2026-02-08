const VOWELS: [char; 5] = ['a', 'e', 'u', 'i', 'o'];

pub fn is_nice(s: &str) -> bool {
    !s.contains("ab")
        && !s.contains("cd")
        && !s.contains("pq")
        && !s.contains("xy")
        && s.chars().filter(|x| VOWELS.contains(x)).count() >= 3
        && s.chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|x| x[0] == x[1])
}

pub fn process(_input: &str) -> usize {
    _input.lines().filter(|x| is_nice(x)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn test_is_nice(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_nice(input), expected);
    }
}
