use crate::parsing::parse_input;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

// hyperfine --warmup 2 "cargo run --bin p2" "cargo run --bin p2_s"
// Benchmark 1: cargo run --bin p2
//   Time (mean ± σ):     118.8 ms ±   3.9 ms    [User: 125.4 ms, System: 27.0 ms]
//   Range (min … max):   114.5 ms … 132.4 ms    22 runs
//
// Benchmark 2: cargo run --bin p2_s
//   Time (mean ± σ):      1.052 s ±  0.016 s    [User: 3.374 s, System: 0.033 s]
//   Range (min … max):    1.031 s …  1.087 s    10 runs
//
// Summary
//   cargo run --bin p2 ran
//     8.85 ± 0.32 times faster than cargo run --bin p2_s
pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    input
        .into_par_iter()
        .map(|(left, right)| {
            let mut total: usize = 0;
            let left_n: usize = left.parse().unwrap();
            let right_n: usize = right.parse().unwrap();

            for id in left_n..=right_n {
                let id_str = id.to_string();
                let half = id_str.len() / 2;
                for limit in 0..half {
                    if id_str.len().rem_euclid(limit + 1) == 0 {
                        let all_match = id_str[0..=limit]
                            .chars()
                            .cycle()
                            .zip(id_str.chars())
                            .all(|(a, b)| a == b);
                        if all_match {
                            total += id;
                            break;
                        }
                    }
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
        assert_eq!(process(fixture), 4174379265);
    }
}
