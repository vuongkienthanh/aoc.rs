use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (a, b) = parse_input(_input);
    (a..=b).filter(|x| is_password(*x)).count()
}

fn is_password(mut a: usize) -> bool {
    let mut has_double = false;
    let mut last = a % 10;
    a /= 10;
    while a > 0 {
        let to_check = a % 10;
        if to_check > last {
            return false;
        }
        has_double = has_double || to_check == last;
        last = to_check;
        a /= 10;
    }
    has_double
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(111111, true)]
    #[case(223450, false)]
    #[case(123789, false)]
    fn test_is_password(#[case] input: usize, #[case] expected: bool) {
        assert_eq!(is_password(input), expected);
    }
}
