use crate::parsing::{Item, parse_input};
use fxhash::FxHashSet;

const ADJ: [usize; 25] = [
    0b100010,
    0b1000101,
    0b10001010,
    0b100010100,
    0b1000001000,
    0b10001000001,
    0b100010100010,
    0b1000101000100,
    0b10001010001000,
    0b100000100010000,
    0b1000100000100000,
    0b10000010001000000,
    0b100010100010000000,
    0b1000100000100000000,
    0b10000010001000000000,
    0b100010000010000000000,
    0b1000101000100000000000,
    0b10001010001000000000000,
    0b100010100010000000000000,
    0b1000001000100000000000000,
    0b1000001000000000000000,
    0b10100010000000000000000,
    0b101000100000000000000000,
    0b1010001000000000000000000,
    0b100010000000000000000000,
];

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut grid = input
        .into_iter()
        .flatten()
        .zip(0..25)
        .fold(0, |acc, (cell, i)| {
            acc | if matches!(cell, Item::Bug) { 1 << i } else { 0 }
        });
    let mut seen = FxHashSet::default();

    loop {
        if !seen.insert(grid) {
            break grid;
        }
        let mut new_grid = 0;
        for i in 0..25 {
            let bug = 1 << i;
            let adj_bugs = (ADJ[i] & grid).count_ones();
            if grid & bug == bug {
                if adj_bugs == 1 {
                    new_grid |= bug;
                }
            } else {
                if adj_bugs == 1 || adj_bugs == 2 {
                    new_grid |= bug;
                }
            }
        }

        grid = new_grid;
    }
}
