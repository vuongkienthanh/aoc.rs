pub mod parsing;
pub mod part1;
pub mod part2;
type Point = (usize, usize);

pub fn calc_area(p1: Point, p2: Point) -> usize {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}
