use crate::parsing::parse_input;
use crate::{left_bound, right_bound};

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    println!("{input:?}");
    // println!("{_rest:?}");
    assert!(_rest.is_empty());

    input
        .into_iter()
        .map(|(l, r)| (left_bound(l), right_bound(r)))
        .filter(|((l, _), (r, _))| l <= r)
        .map(|((l, ll), (r, rl))| {
            assert_eq!(ll, rl);
            (l, r, ll, take_left_half(l, ll))
        })
        .map(|(l, r, ll, half)| {
            // println!("l={l} r={r} half={half}");
            let mut half = half;
            let mut double = double_the_half(half, ll);
            while double < l {
                half += 1;
                double = double_the_half(half, ll);
            }
            let mut sum = 0;
            while double <= r {
                // println!("found {double}");
                sum += double;
                half += 1;
                double = double_the_half(half, ll);
            }
            sum
        })
        .sum()
}

fn take_left_half(mut a: usize, l: usize) -> usize {
    for _ in 0..(l / 2) {
        a /= 10;
    }
    a
}
fn double_the_half(a: usize, l: usize) -> usize {
    a * 10usize.pow(l as u32 / 2) + a
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 1227775554);
    }

    #[rstest]
    #[case(54, 2, 5)]
    #[case(1234, 4, 12)]
    fn test_take_left_half(#[case] input: usize, #[case] len: usize, #[case] expect: usize) {
        assert_eq!(take_left_half(input, len), expect);
    }
    #[rstest]
    #[case(54, 4, 5454)]
    #[case(1234, 8, 12341234)]
    fn test_double_the_half(#[case] input: usize, #[case] len: usize, #[case] expect: usize) {
        assert_eq!(double_the_half(input, len), expect);
    }
}
