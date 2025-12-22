use crate::parsing::{Bracket, parse_input};
use fxhash::FxHashSet as Set;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    input.into_iter().filter(|x| is_SSL_supported(x)).count()
}

#[allow(non_snake_case)]
fn get_all_ABA(input: &str) -> Vec<&str> {
    let mut ans = vec![];
    for i in 0..input.len() - 2 {
        let s = &input[i..i + 3];
        let mut si = s.chars();
        let first = si.next().unwrap();
        let second = si.next().unwrap();
        let third = si.next().unwrap();
        if first == third && first != second {
            ans.push(s);
        }
    }
    ans
}

#[allow(non_snake_case)]
fn ABA_to_BAB(input: &str) -> String {
    let mut ans = String::new();
    let mut i = input.chars();
    let first = i.next().unwrap();
    let second = i.next().unwrap();
    ans.push(second);
    ans.push(first);
    ans.push(second);
    ans
}

#[allow(non_snake_case)]
fn is_SSL_supported<'a>(input: &[Bracket<'a>]) -> bool {
    let mut v_in = vec![];
    let mut v_out = vec![];
    for b in input {
        match b {
            Bracket::In(x) => v_in.push(x),
            Bracket::Out(x) => v_out.push(x),
        }
    }
    let mut all_ABA = Set::default();
    for b_out in v_out {
        for aba in get_all_ABA(b_out) {
            all_ABA.insert(aba);
        }
    }
    for aba in all_ABA {
        for b_in in &v_in {
            if b_in.contains(&ABA_to_BAB(aba)) {
                return true;
            }
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_line;
    use rstest::*;

    #[rstest]
    #[case("aba[bab]xyz", true)]
    #[case("xyx[xyx]xyx", false)]
    #[case("aaa[kek]eke", true)]
    #[case("zazbz[bzb]cdb", true)]
    fn test_is_SSL_supported(#[case] input: &str, #[case] expected: bool) {
        let (_, v) = parse_line(input).unwrap();
        assert_eq!(is_SSL_supported(&v), expected);
    }
}
