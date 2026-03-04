use md5::{Digest, Md5};

pub fn process(_input: &str) -> usize {
    (0..usize::MAX)
        .find(|i| {
            let d = Md5::digest(format!("{_input}{i}"));
            d[0..2] == [0, 0] && d[2] < 17
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
