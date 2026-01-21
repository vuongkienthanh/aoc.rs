use aoc_helper::direction::{Direction, step};
use intcode::{Computer, parse};
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut map = BTreeMap::new();
    let (mut loc, mut dir) = ((0isize, 0isize), Direction::Up);
    map.insert(loc, 0);
    let mut comp = Computer::new(input);
    comp.append_input(0);

    while let Some(paint) = comp.long_run() {
        map.insert(loc, paint);
        let turn = comp.long_run().unwrap();
        match turn {
            0 => {
                dir = dir.turn_left();
                let (x, y) = step(dir);
                loc = (loc.0 + x, loc.1 + y);
            }
            1 => {
                dir = dir.turn_right();
                let (x, y) = step(dir);
                loc = (loc.0 + x, loc.1 + y);
            }
            _ => panic!(),
        }
        comp.append_input(*map.entry(loc).or_default());
    }
    map.len()
}
