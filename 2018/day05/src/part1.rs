pub fn process(_input: &str) -> usize {
    react(_input).len()
}

fn react(input: &str) -> Vec<u8> {
    let mut left: Vec<u8> = Vec::new();
    let mut right: Vec<u8> = Vec::new();
    right.extend(input.bytes().rev());

    while let Some(r) = right.pop() {
        if let Some(l) = left.pop() {
            if l.abs_diff(r) != 32 {
                left.push(l);
                left.push(r);
            }
        } else {
            left.push(r);
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
            react(input)
                .into_iter()
                .map(|x| x as char)
                .collect::<String>(),
            expected.to_string()
        );
    }
}
