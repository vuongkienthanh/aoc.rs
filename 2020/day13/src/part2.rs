use crate::parse;
use aoc_helper::algebra::chinese_remainder_theorem::{Item, find_x};
pub fn process(_input: &str) -> usize {
    let (_, buses) = parse(_input);
    let buses = buses
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| {
            b.map(|b| Item {
                modulus: b,
                rem: (b * i - i) % b,
            })
        })
        .collect();

    find_x(buses)
}
