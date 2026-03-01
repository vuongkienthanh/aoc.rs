// https://github.com/EmilOhlsson/advent-of-code/blob/main/rust/2021/24-arithmetic-logic-unit/notes.svg
use crate::parsing::{Group, parse_input};

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    let mut w = [0; 14];
    let mut stack = vec![];

    for (i, g) in input.into_iter().enumerate() {
        match g {
            Group::Div1(a) => stack.push((i, a)),
            Group::Div26(b) => {
                let (ia, a) = stack.pop().unwrap();
                let diff = a + b;
                w[ia] = 9.min(9 - diff);
                w[i] = 9.min(9 + diff);
            }
        }
    }

    w.into_iter().reduce(|a, b| a * 10 + b).unwrap()
}
