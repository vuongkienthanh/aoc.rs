pub mod parsing;
pub mod part1;
pub mod part2;

pub fn left_bound(a: &str) -> (usize, usize) {
    let ans: usize;
    let mut len = a.len();
    if !a.len().is_multiple_of(2) {
        ans = 10usize.pow(len as u32) + 10usize.pow(len as u32 / 2);
        len += 1;
    } else {
        ans = a.parse().unwrap();
    }
    (ans, len)
}

pub fn right_bound(a: &str) -> (usize, usize) {
    let mut ans = 0;
    let mut len = a.len();
    if !a.len().is_multiple_of(2) {
        len = a.len() - 1;
        for _ in 0..len {
            ans = ans * 10 + 9;
        }
    } else {
        ans = a.parse().unwrap();
    }
    (ans, len)
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("5", (11,2))]
    #[case("123", (1010, 4))]
    #[case("1234", (1234, 4))]
    fn test_left_bound(#[case] input: &str, #[case] expect: (usize, usize)) {
        assert_eq!(left_bound(input), expect);
    }

    #[rstest]
    #[case("5", (0, 0))]
    #[case("123", (99,2))]
    #[case("1234", (1234,4))]
    fn test_right_bound(#[case] input: &str, #[case] expect: (usize, usize)) {
        assert_eq!(right_bound(input), expect);
    }
}
