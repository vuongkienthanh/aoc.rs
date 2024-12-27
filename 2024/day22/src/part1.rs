use crate::evolve;
pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|mut x| {
            for _ in 0..2000 {
                x = evolve(x)
            }
            x
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "reason"]
    fn test_process() {
        let input = r#"1
10
100
2024"#;
        assert_eq!(process(input), 37327623);
    }
}
