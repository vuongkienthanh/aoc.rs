use crate::parsing::parse_input;

pub fn process(_input: &str) -> u32 {
    let input = parse_input(_input);
    input
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .map(|line| line.bytes().fold(0u32, |acc, c| acc | (1 << c - b'a')))
                .reduce(|a, b| a | b)
                .unwrap()
                .count_ones()
        })
        .sum()
}
