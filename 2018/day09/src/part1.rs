use crate::{game, parsing::parse_input};

pub fn process(_input: &str) -> usize {
    let (players, max) = parse_input(_input);
    game(players, max)
}
