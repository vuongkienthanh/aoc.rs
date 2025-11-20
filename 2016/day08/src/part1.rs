use crate::parsing::parse_input;
use crate::{COLS, ROWS, swipe};

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());
    // println!("{input:?}");

    let mut screen = [[0; COLS]; ROWS];

    swipe(&mut screen, input);
    screen
        .into_iter()
        .map(|x| x.into_iter().sum::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#""#
    }
    #[rstest]
    fn test_process_1(fixture: &str) {
        assert_eq!(process(fixture), 0);
    }

    #[rstest]
    #[case("", 0)]
    fn test_process_2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
