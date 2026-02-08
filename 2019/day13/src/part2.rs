use crate::Tile;
use intcode::{Computer, RunResult, parse};
use std::cmp::Ordering;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut comp = Computer::new(input);
    *comp.prog.get_mut(&0).unwrap() = 2;
    let (mut score, mut ball, mut paddle) = (0, 0, 0);

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
                    score = comp.long_run().output();
                } else {
                    let tile: Tile = comp.long_run().output().into();
                    match tile {
                        Tile::ball => ball = x,
                        Tile::paddle => paddle = x,
                        _ => (),
                    }
                }
            }
        }
    }
    score as usize
}
