use crate::{Point, generate};
use aoc_helper::adj::naive::adj8;
use fxhash::FxHashMap as Map;

pub fn process(_input: &str) -> usize {
    let input: usize = _input.parse().unwrap();
    let mut map: Map<Point, usize> = Map::default();
    map.insert((0, 0), 1);
    for layer in 1.. {
        for p in generate(layer) {
            let val: usize = adj8(p).into_iter().filter_map(|x| map.get(&x)).sum();
            if val > input {
                return val;
            }
            map.insert(p, val);
        }
    }

    panic!("should have an answer")
}
