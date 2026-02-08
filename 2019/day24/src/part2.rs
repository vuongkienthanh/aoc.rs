use crate::parsing::{Item, parse_input};
use fxhash::FxHashMap;

const ADJ: [[usize; 3]; 25] = [
    [0, 0b100010, 0b100010000000],
    [0, 0b1000101, 0b10000000],
    [0, 0b10001010, 0b10000000],
    [0, 0b100010100, 0b10000000],
    [0, 0b1000001000, 0b10000010000000],
    [0, 0b10001000001, 0b100000000000],
    [0, 0b100010100010, 0],
    [0b11111, 0b101000100, 0],
    [0, 0b10001010001000, 0],
    [0, 0b100000100010000, 0b10000000000000],
    [0, 0b1000100000100000, 0b100000000000],
    [0b100001000010000100001, 0b10000010001000000, 0],
    [0, 0b100010100010000000, 0],
    [0b1000010000100001000010000, 0b1000100000100000000, 0],
    [0, 0b10000010001000000000, 0b10000000000000],
    [0, 0b100010000010000000000, 0b100000000000],
    [0, 0b1000101000100000000000, 0],
    [0b1111100000000000000000000, 0b10001010000000000000000, 0],
    [0, 0b100010100010000000000000, 0],
    [0, 0b1000001000100000000000000, 0b10000000000000],
    [0, 0b1000001000000000000000, 0b100000100000000000],
    [0, 0b10100010000000000000000, 0b100000000000000000],
    [0, 0b101000100000000000000000, 0b100000000000000000],
    [0, 0b1010001000000000000000000, 0b100000000000000000],
    [0, 0b100010000000000000000000, 0b100010000000000000],
];

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut grid3d = FxHashMap::default();
    grid3d.insert(
        0,
        input
            .into_iter()
            .flatten()
            .zip(0..25)
            .fold(0usize, |acc, (cell, i)| {
                acc | if matches!(cell, Item::Bug) { 1 << i } else { 0 }
            }),
    );
    for _ in 0..200 {
        let mut new_grid3d = FxHashMap::default();
        let min = grid3d.keys().min().cloned().unwrap() - 1;
        let max = grid3d.keys().max().cloned().unwrap() + 1;

        for lvl in min..=max {
            let grid = grid3d.get(&lvl).unwrap_or(&0);
            for i in 0..25 {
                if i == 12 {
                    continue;
                }
                let bug = 1 << i;
                let adj_bugs = ADJ[i]
                    .iter()
                    .zip([-1, 0, 1])
                    .map(|(x, d)| (grid3d.get(&(lvl + d)).unwrap_or(&0) & x).count_ones())
                    .sum::<u32>();
                if grid & bug == bug {
                    if adj_bugs == 1 {
                        *new_grid3d.entry(lvl).or_default() |= bug;
                    }
                } else {
                    if adj_bugs == 1 || adj_bugs == 2 {
                        *new_grid3d.entry(lvl).or_default() |= bug;
                    }
                }
            }
        }

        grid3d = new_grid3d;
    }
    grid3d.into_values().map(|x| x.count_ones() as usize).sum()
}
