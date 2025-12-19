use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    parse_input(_input)
        .into_iter()
        .fold((0, 50), |(mut count, mut loc), x| {
            // when at zero and turn left, don't increase count
            if loc == 0 && x < 0 {
                count -= 1;
            }
            loc += x;
            if loc == 0 {
                count += 1;
            } else if loc < 0 {
                count += (loc / 100).unsigned_abs() + 1;
                loc = ((loc % 100) + 100) % 100;
            } else if loc > 99 {
                count += (loc / 100) as usize;
                loc %= 100;
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
        assert_eq!(process(fixture), 6);
    }
}
