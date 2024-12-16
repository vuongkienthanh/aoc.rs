use super::{find_token, parse};
pub fn process(_input: &str) -> isize {
    let (_, v) = parse(_input).unwrap();
    v.into_iter()
        .filter_map(|(a, b, p)| find_token(a, b, p))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        assert_eq!(process(input), 480);
    }
}
