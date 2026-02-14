use crate::parsing::{Item, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (mut x, mut depth, mut aim) = (0, 0, 0);
    for item in input {
        match item {
            Item::F(i) => {
                x += i;
                depth += aim * i;
            }
            Item::U(i) => aim -= i,
            Item::D(i) => aim += i,
        }
    }
    x * depth    
}
