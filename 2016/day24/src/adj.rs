use crate::parsing::Point;

pub fn neg_dir(dir: usize) -> usize {
    match dir {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        _ => panic!("no such direction"),
    }
}

pub fn up(p: Point) -> Point {
    (p.0 - 1, p.1)
}
pub fn down(p: Point) -> Point {
    (p.0 + 1, p.1)
}
pub fn left(p: Point) -> Point {
    (p.0, p.1 - 1)
}
pub fn right(p: Point) -> Point {
    (p.0, p.1 + 1)
}
pub fn adj4(p: Point) -> [Point; 4] {
    [up(p), down(p), left(p), right(p)]
}
