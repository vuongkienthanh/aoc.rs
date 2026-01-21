use crate::Tile;
use intcode::{Computer, RunResult, parse};
use std::cmp::Ordering;
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut comp = Computer::new(input);
    *comp.prog.get_mut(&0).unwrap() = 2;
    let mut map = BTreeMap::new();
    let mut score = 0;
    let mut ball = 0;
    let mut paddle = 0;

    loop {
        match comp.long_run() {
            RunResult::Halt => break,
            RunResult::WaitingInput => comp.input(match paddle.cmp(&ball) {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            }),
            RunResult::Output(x) => {
                let y = comp.long_run().output();
                if (x, y) == (-1, 0) {
                    score = if let RunResult::Output(score) = comp.long_run() {
                        score as usize
                    } else {
                        panic!("should have score")
                    }
                } else {
                    let (x, y) = (x as usize, y as usize);
                    let tile: Tile = comp.long_run().output().into();
                    map.insert((x, y), tile.clone());
                    if matches!(tile.clone(), Tile::ball) {
                        ball = x;
                    }
                    if matches!(tile, Tile::paddle) {
                        paddle = x;
                    }
                }
            }
        }
    }
    score
}
