use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn process(_input: &str) -> usize {
    _input
        .par_lines()
        .map(|line| {
            let line: Vec<(usize, usize)> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .collect();

            let (mut idx, mut max) = (0, 0);

            for d in (0..12).rev() {
                let (new_idx, new_max) = line[idx..line.len() - d]
                    .iter()
                    .rev()
                    .max_by_key(|(_, e)| *e)
                    .cloned()
                    .unwrap();
                idx = new_idx + 1;
                max = max * 10 + new_max;
            }

            max
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"987654321111111
811111111111119
234234234234278
818181911112111"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 3121910778619);
    }
}
