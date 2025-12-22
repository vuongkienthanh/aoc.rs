use crate::parsing::parse_input;
use crate::{build_all_combinations, score};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    build_all_combinations(input.len())
        .iter()
        .filter(|v| {
            v.iter()
                .zip(input.iter())
                .map(|(q, i)| i[4] * *q as isize)
                .sum::<isize>()
                == 500
        })
        .map(|v| score(v, &input))
        .max()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#
    }
    #[rstest]
    fn test_process(fixture: &str) {
        assert_eq!(process(fixture), 57600000);
    }
}
