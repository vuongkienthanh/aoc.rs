use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let mut input: Vec<usize> = parse_input(_input)
        .into_iter()
        .map(|v| v.into_iter().sum())
        .collect();
    input.sort_unstable();
    input.into_iter().rev().take(3).sum()
}
