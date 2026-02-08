use crate::parsing::{Bracket, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    input.into_iter().filter(|x| is_TLS_supported(x)).count()
}
#[allow(non_snake_case)]
fn is_ABBA(input: &str) -> bool {
    input
        .as_bytes()
        .windows(4)
        .any(|v| v[0] != v[1] && v[0] == v[3] && v[1] == v[2])
}

#[allow(non_snake_case)]
fn is_TLS_supported<'a>(input: &[Bracket<'a>]) -> bool {
    let mut is_ok = false;
    for b in input {
        match b {
            Bracket::In(s) => {
                if is_ABBA(s) {
                    return false;
                }
            }
            Bracket::Out(s) => {
                if is_ABBA(s) {
                    is_ok = true
                }
            }
        }
    }
    is_ok
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_line;
    use rstest::*;

    #[rstest]
    #[case("abba[mnop]qrst", true)]
    #[case("abcd[bddb]xyyx", false)]
    #[case("aaaa[qwer]tyui", false)]
    #[case("ioxxoj[asdfgh]zxcvbn", true)]
    fn test_is_TLS_supported(#[case] input: &str, #[case] expected: bool) {
        let (_, v) = parse_line(input).unwrap();
        assert_eq!(is_TLS_supported(&v), expected);
    }
}
