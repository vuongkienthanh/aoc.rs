use crate::build_map;
use intcode::{Computer, parse};
use aoc_helper::adj::checked_u::adj4;

pub fn process(_input: &str) -> usize {
    let camera = Computer::new(parse(_input));
    let (map, _) = build_map(camera);
    map.iter()
        .filter(|p| adj4(**p).into_iter().flatten().all(|p2| map.contains(&p2)))
        .map(|(a, b)| a * b)
        .sum()
}
