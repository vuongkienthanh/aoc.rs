use crate::forward;
use crate::parsing::{Turn, parse_input};
use aoc_helper::direction::Direction;
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut seen: HashSet<(isize, isize)> = HashSet::new();

    let mut coord = (0isize, 0isize);
    seen.insert(coord);
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
        let new_coord = forward(dir, coord, i);
        match dir {
            Direction::Up => {
                for row in new_coord.0..coord.0 {
                    if !seen.insert((row, coord.1)) {
                        return (row.abs() + coord.1.abs()).try_into().unwrap();
                    }
                }
            }
            Direction::Down => {
                for row in coord.0 + 1..=new_coord.0 {
                    if !seen.insert((row, coord.1)) {
                        return (row.abs() + coord.1.abs()).try_into().unwrap();
                    }
                }
            }
            Direction::Left => {
                for col in new_coord.1..coord.1 {
                    if !seen.insert((coord.0, col)) {
                        return (coord.0.abs() + col.abs()).try_into().unwrap();
                    }
                }
            }
            Direction::Right => {
                for col in coord.1 + 1..=new_coord.1 {
                    if !seen.insert((coord.0, col)) {
                        return (coord.0.abs() + col.abs()).try_into().unwrap();
                    }
                }
            }
        }
        coord = new_coord;
    }
    panic!("no answer")
}
