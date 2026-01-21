use crate::{Tile, screen};
use intcode::{Computer, RunResult, parse};
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut comp = Computer::new(input);
    let mut map = BTreeMap::new();

    loop {
        match comp.long_run() {
            RunResult::Halt => break,
            RunResult::WaitingInput => panic!("should not have any input"),
            RunResult::Output(x) => {
                let y = comp.long_run().output();
                let tile: Tile = comp.long_run().output().into();

                map.insert((x as usize, y as usize), tile);
            }
        }
    }
    screen(&map);

    map.values().filter(|x| matches!(x, Tile::block)).count()
}
