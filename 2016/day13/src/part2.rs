use crate::{Coord, is_wall};
use aoc_helper::adj::checked_u::adj4;
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let input = _input.parse::<usize>().unwrap();
    run(input)
}
fn run(input: usize) -> usize {
    let mut ans = 1;
    let mut v: Vec<Coord> = Vec::new();
    let mut seen: HashSet<Coord> = HashSet::new();
    seen.insert((1, 1));
    v.push((1, 1));

    for _ in 0..50 {
        let mut new_v = Vec::new();
        for c in v {
            for adj in adj4(c).into_iter().flatten() {
                if seen.insert(adj) && !is_wall(input, adj) {
                    ans += 1;
                    new_v.push(adj);
                }
            }
        }

        v = new_v;
    }
    ans
}
