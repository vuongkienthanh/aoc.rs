use crate::parsing::{Item, parse_input};
use aoc_helper::direction::Direction;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut ship = (0isize, 0isize, Direction::Right);
    for item in input {
        match item {
            Item::N(i) => ship.1 -= i,
            Item::S(i) => ship.1 += i,
            Item::E(i) => ship.0 += i,
            Item::W(i) => ship.0 -= i,
            Item::L(i) => {
                for _ in 0..i / 90 {
                    ship.2 = ship.2.turn_left();
                }
            }
            Item::R(i) => {
                for _ in 0..i / 90 {
                    ship.2 = ship.2.turn_right();
                }
            }
            Item::F(i) => match ship.2 {
                Direction::Up => ship.1 -= i,
                Direction::Down => ship.1 += i,
                Direction::Left => ship.0 -= i,
                Direction::Right => ship.0 += i,
            },
        }
    }
    ship.0.unsigned_abs() + ship.1.unsigned_abs()
}
