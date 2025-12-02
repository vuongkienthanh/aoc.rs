use crate::multi;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    // println!("{input:?}");
    // println!("{_rest:?}");
    assert!(_rest.is_empty());

    let mut sum = 0;
    for (left, right) in input {
        let left_n: usize = left.parse().unwrap();
        let right_n: usize = right.parse().unwrap();

        for len in left.len() as u32..=right.len() as u32 {
            if !len.is_multiple_of(2) {
                continue;
            }
            let left_bound = 10usize.pow(len - 1).max(left_n);
            let right_bound = (10usize.pow(len) - 1).min(right_n);

            let part_len = len / 2;
            let num = 10usize.pow(part_len - 1);
            for j in 0.. {
                let expand = multi(num + j, part_len, 2);
                if expand < left_bound {
                    continue;
                }
                if expand > right_bound {
                    break;
                }
                sum += expand
            }
        }
    }
    sum
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
}
