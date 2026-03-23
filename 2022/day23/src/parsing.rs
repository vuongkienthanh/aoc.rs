pub type Point = (isize, isize);
use std::collections::BTreeSet;

pub fn parse_input(input: &str) -> BTreeSet<Point> {
    let mut set = BTreeSet::new();
    for (r, line) in input.lines().enumerate() {
        for (c, cell) in line.chars().enumerate() {
            if cell == '#' {
                set.insert((r as isize, c as isize));
            }
        }
    }
    set
}