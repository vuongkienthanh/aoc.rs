use crate::multi;
use crate::parsing::parse_input;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    input
        .into_par_iter()
        .map(|(left, right)| {
            let mut set: HashSet<usize> = HashSet::new();
            let left_n: usize = left.parse().unwrap();
            let right_n: usize = right.parse().unwrap();

            for len in left.len()..=right.len() {
                let len = len as u32;
                let left_bound = 10usize.pow(len - 1).max(left_n);
                let right_bound = (10usize.pow(len) - 1).min(right_n);

                for part_len in 1..=len / 2 {
                    if !len.is_multiple_of(part_len) {
                        continue;
                    }
                    let times = len / part_len;
                    let num = 10usize.pow(part_len - 1);
                    for j in 0.. {
                        let expand = multi(num + j, part_len, times);
                        if expand < left_bound {
                            continue;
                        }
                        if expand > right_bound {
                            break;
                        }
                        set.insert(expand);
                    }
                }
            }
            set.into_iter().sum::<usize>()
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
        assert_eq!(process(fixture), 4174379265);
    }
}
