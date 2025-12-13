use crate::multi;
use crate::parsing::parse_input;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    input
        .into_par_iter()
        .map(|(left, right)| {
            let left_n: usize = left.parse().unwrap();
            let right_n: usize = right.parse().unwrap();

            let mut total = 0usize;
            for len in left.len()..=right.len() {
                if !len.is_multiple_of(2) {
                    continue;
                }
                let len = len as u32;
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
                    total += expand
                }
            }
            total
        })
        .sum()
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
