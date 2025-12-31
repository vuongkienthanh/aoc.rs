use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    react(_input).len()
}

fn react(input: &str) -> VecDeque<u8> {
    let mut left: VecDeque<u8> = VecDeque::new();
    let mut right: VecDeque<u8> = VecDeque::new();
    right.extend(input.bytes());

    while let Some(r) = right.pop_front() {
        if let Some(l) = left.pop_back() {
            if l.abs_diff(r) != 32 {
                left.push_back(l);
                left.push_back(r);
            }
        } else {
            left.push_back(r);
        }
    }

    left
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("dabAcCaCBAcCcaDA", "dabCBAcaDA")]
    #[case("aA", "")]
    #[case("abBA", "")]
    #[case("abAB", "abAB")]
    #[case("aabAAB", "aabAAB")]
    fn test_react(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            react(input).into_iter().map(|x| x as char).collect::<String>(),
            expected.to_string()
        );
    }
}
