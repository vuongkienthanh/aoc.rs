use crate::parsing::parse_input;

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    input.into_iter().sum()
}
