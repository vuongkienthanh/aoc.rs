use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (a, b) = parse_input(_input);
    (a..=b).filter(|x| is_password(*x)).count()
}
fn is_password(mut a: usize) -> bool {
    let mut last = a % 10;
    a /= 10;
    let mut has_double = false;
    let mut running = 1;
    while a > 0 {
        let to_check = a % 10;
        if to_check > last {
            return false;
        }
        if !has_double {
            if to_check == last {
                running += 1;
            } else {
                has_double = running == 2;
                running = 1;
            }
        }
        a /= 10;
        last = to_check;
    }
    has_double || running == 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(112233, true)]
    #[case(123444, false)]
    #[case(111122, true)]
    fn test_is_password(#[case] input: usize, #[case] expected: bool) {
        assert_eq!(is_password(input), expected);
    }
}
