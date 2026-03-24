use crate::parsing::parse_input;
use crate::{State, branch_and_bound};

pub fn process(_input: &str) -> u32 {
    parse_input(_input)
        .into_iter()
        .take(3)
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(&blueprint, State::new(32), &mut best);
            best as u32
        })
        .product()
}
