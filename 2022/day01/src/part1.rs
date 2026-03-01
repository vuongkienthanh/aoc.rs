use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input.into_iter().map(|v| v.into_iter().sum()).max().unwrap()
}
