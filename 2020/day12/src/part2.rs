use crate::parsing::{Item, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut ship = (0isize, 0isize);
    let mut waypoint = (10isize, -1isize);
    for item in input {
        match item {
            Item::N(i) => waypoint.1 -= i,
            Item::S(i) => waypoint.1 += i,
            Item::E(i) => waypoint.0 += i,
            Item::W(i) => waypoint.0 -= i,
            Item::L(i) => {
                for _ in 0..i / 90 {
                    waypoint = (waypoint.1, -waypoint.0);
                }
            }
            Item::R(i) => {
                for _ in 0..i / 90 {
                    waypoint = (-waypoint.1, waypoint.0);
                }
            }
            Item::F(i) => {
                ship.0 += waypoint.0 * i;
                ship.1 += waypoint.1 * i;
            }
        }
    }
    ship.0.unsigned_abs() + ship.1.unsigned_abs()
}
