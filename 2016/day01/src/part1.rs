use crate::forward;
use crate::parsing::{Turn, parse_input};
use aoc_helper::direction::Direction;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut coord = (0isize, 0isize);
    let mut dir = Direction::Up;
    for (t, i) in input {
        match t {
            Turn::Left => {
                dir = dir.turn_left();
            }
            Turn::Right => {
                dir = dir.turn_right();
            }
        }
        coord = forward(dir, coord, i);
    }

    (coord.0.abs() + coord.1.abs()).try_into().unwrap()
}
