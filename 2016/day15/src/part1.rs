use crate::parsing::parse_input;
use aoc_helper::algebra::chinese_remainder_theorem::find_x;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    find_x(input)
}
