use crate::parsing::parse_input;
use aoc_helper::algebra::chinese_remainder_theorem::{Item, find_x};

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input);
    input.push(Item {
        modulus: 11,
        rem: 11 - ((input.len() + 1) % 11),
    });
    find_x(input)
}
