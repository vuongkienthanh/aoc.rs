use crate::parsing::{Item, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (mut x, mut depth) = (0, 0);
    for item in input {
        match item {
            Item::F(i) => x += i,
            Item::U(i) => depth -= i,
            Item::D(i) => depth += i,
        }
    }
    x * depth
}
