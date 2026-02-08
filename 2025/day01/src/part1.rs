use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    parse_input(_input)
        .into_iter()
        .fold((0, 50), |(mut count, mut loc), x| {
            loc += x;
            loc = ((loc % 100) + 100) % 100;

            if loc == 0 {
                count += 1;
            }
            (count, loc)
        })
        .0
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 3);
    }
}
