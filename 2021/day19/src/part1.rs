use crate::normalize;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (water,_) = normalize(input);
    water.len()
}
