use crate::{Coord, adj4, is_wall};
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

    for i in 0..50 {
        let mut new_v = Vec::new();
        for c in v {
            for adj in adj4(c).into_iter().filter_map(|x| x) {
                if seen.insert(adj) && !is_wall(input, adj) {
                    if i % 2 == 1 {
                        ans += 1;
                    }
                    new_v.push(adj);
                }
            }
        }

        v = new_v;
    }
    ans
}
