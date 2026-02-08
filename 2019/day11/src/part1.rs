use aoc_helper::direction::{Direction, step};
use intcode::{Computer, RunResult, parse};
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut map = BTreeMap::new();
    let (mut loc, mut dir) = ((0isize, 0isize), Direction::Up);
    map.insert(loc, 0);
    let mut comp = Computer::new(input);
    comp.input(0);

    loop {
        match comp.long_run() {
            RunResult::Halt => break,
            RunResult::WaitingInput => {
                comp.input(*map.entry(loc).or_default());
            }
            RunResult::Output(paint) => {
                map.insert(loc, paint);
                if let RunResult::Output(turn) = comp.long_run() {
                    dir = match turn {
                        0 => dir.turn_left(),
                        1 => dir.turn_right(),
                        _ => panic!(),
                    };
                    let (x, y) = step(dir);
                    loc = (loc.0 + x, loc.1 + y);
                } else {
                    panic!();
                }
            }
        }
    }

    map.len()
}
