use crate::parsing::parse_input;
use crate::{left_bound, right_bound};

pub fn process(_input: &str) -> usize {
    let (_, input) = parse_input(_input).unwrap();

    input
        .into_iter()
        .map(|(l, r)| {
            (
                left_bound(l),
                right_bound(r),
                sum_odd_left(l) + sum_odd_right(r),
            )
        })
        .filter(|((l, _), (r, _), _)| l <= r)

    // 0
}

fn sum_odd_left(a: &str) -> usize {
    if a.len().is_multiple_of(2) {
        0
    } else {
        let mut sum = 0;
        let left = a.parse::<usize>().unwrap();
        for i in 1..10 {
            let mut n = 0;
            for _ in 0..a.len() {
                n = n * 10 + i;
            }
            if n >= left {
                sum += n;
            }
        }
        sum
    }
}
fn sum_odd_right(a: &str) -> usize {
    if a.len().is_multiple_of(2) {
        0
    } else {
        let mut sum = 0;
        let right = a.parse::<usize>().unwrap();
        for i in 1..10 {
            let mut n = 0;
            for _ in 0..a.len() {
                n = n * 10 + i;
            }
            if n <= right {
                sum += n;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#
    }
    // #[rstest]
    // fn test_process_(fixture: &str) {
    //     assert_eq!(process(fixture), 4174379265);
    // }
    #[rstest]
    #[case("95", 0)]
    #[case("998", 999)]
    fn test_sum_odd_left(#[case] input: &str, #[case] expect: usize) {
        assert_eq!(sum_odd_left(input), expect);
    }
    #[rstest]
    #[case("1012", 0)]
    #[case("115", 111)]
    fn test_sum_odd_right(#[case] input: &str, #[case] expect: usize) {
        assert_eq!(sum_odd_right(input), expect);
    }
}
