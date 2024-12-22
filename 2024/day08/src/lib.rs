pub mod part1;
pub mod part2;

use glam::IVec2;
use std::collections::HashMap;

/// return rows, cols, and compiled map
fn parse(input: &str) -> (i32, i32, HashMap<char, Vec<IVec2>>) {
    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().chars().count() as i32;
    (
        rows,
        cols,
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .map(move |(x, c)| (x as i32, y as i32, c))
            })
            .filter(|(_, _, c)| *c != '.')
            .fold(HashMap::new(), |mut acc, (x, y, c)| {
                acc.entry(c).or_default().push(IVec2::new(x, y));
                acc
            }),
    )
}
