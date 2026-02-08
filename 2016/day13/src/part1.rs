use crate::{Coord, is_wall};
use aoc_helper::adj::checked_u::adj4;
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let input = _input.parse::<usize>().unwrap();

    run(input, (31, 39))
}

fn run(input: usize, target: Coord) -> usize {
    let mut seen: HashSet<Coord> = HashSet::new();
    let mut v: Vec<Coord> = Vec::new();
    seen.insert((1, 1));
    v.push((1, 1));

    let mut step = 0;
    loop {
        let mut new_v = Vec::new();
        step += 1;
        for c in v {
            for adj in adj4(c).into_iter().flatten() {
                if seen.insert(adj) && !is_wall(input, adj) {
                    if adj == target {
                        return step;
                    }
                    new_v.push(adj);
                }
            }
        }

        v = new_v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(10, (7,4), 11)]
    fn test_run(#[case] input: usize, #[case] target: Coord, #[case] expected: usize) {
        assert_eq!(run(input, target), expected);
    }
}
