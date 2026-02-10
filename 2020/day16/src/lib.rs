pub mod parsing;
pub mod part1;
pub mod part2;
use aoc_helper::range::merge;
fn valid_ranges(class: &[(&str, ((usize, usize), (usize, usize)))]) -> Vec<(usize, usize)> {
    let mut ranges = vec![];
    for (_, (a, b)) in class {
        ranges.push(*a);
        ranges.push(*b);
    }
    merge(ranges)
}
