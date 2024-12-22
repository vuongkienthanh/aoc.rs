pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(lhs, rhs)| {
            (
                lhs.parse::<usize>().unwrap(),
                rhs.split_ascii_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(lhs, rhs)| calculate(rhs).any(|x| *lhs == x))
        .map(|(lhs, _)| lhs)
        .sum()
}

fn calculate(right: &[usize]) -> Box<dyn Iterator<Item = usize> + '_> {
    match right.len() {
        1 => Box::new([right[0]].into_iter()),
        _ => Box::new(
            calculate(&right[..right.len() - 1])
                .flat_map(|rest| [rest + right[right.len() - 1], rest * right[right.len() - 1]]),
        ),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(process(input), 3749);
    }
}
