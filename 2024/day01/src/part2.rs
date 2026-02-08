use std::collections::HashMap;
pub fn process(_input: &str) -> usize {
    let (left, right) = _input
        .lines()
        .map(|s| {
            let mut w = s.split_ascii_whitespace();
            (
                w.next().unwrap().parse::<usize>().unwrap(),
                w.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut left, mut right), ele| {
                left.push(ele.0);
                right.entry(ele.1).and_modify(|x| *x += 1).or_insert(1);
                (left, right)
            },
        );

    left.into_iter()
        .map(|x| x * right.get(&x).cloned().unwrap_or(0))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(process(input), 31);
    }
}
