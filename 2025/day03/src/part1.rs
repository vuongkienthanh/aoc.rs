use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn process(_input: &str) -> u32 {
    _input
        .par_lines()
        .map(|line| {
            let line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut max = 0;
            let mut second = 0;

            for i in 0..line.len() - 1 {
                if line[i] > max {
                    max = line[i];
                    second = line[i + 1];
                    continue;
                }
                if line[i] > second {
                    second = line[i];
                    continue;
                }
            }
            if line.last().unwrap() > &second {
                second = line.last().copied().unwrap();
            }
            max * 10 + second
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
        assert_eq!(process(fixture), 357);
    }
}
