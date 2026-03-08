use crate::parsing::parse_input;
use crate::{radius_map, range_at};
use fxhash::FxHashSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let at = 2_000_000;
    let objs = input
        .iter()
        .flat_map(|(sensor, beacon)| [sensor, beacon])
        .filter(|(_, y)| *y == at)
        .collect::<FxHashSet<_>>()
        .len();
    let map = radius_map(input);
    range_at(&map, at)
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum::<isize>() as usize
        - objs
}
