use crate::adj::{Adj4, Adj8};

pub type Point = (usize, usize);

pub fn top(p: Point, _rows: usize, _cols: usize) -> Option<Point> {
    if p.0 > 0 { Some((p.0 - 1, p.1)) } else { None }
}
pub fn bottom(p: Point, rows: usize, _cols: usize) -> Option<Point> {
    (p.0 < rows - 1).then_some((p.0 + 1, p.1))
}
pub fn left(p: Point, _rows: usize, _cols: usize) -> Option<Point> {
    if p.1 > 0 { Some((p.0, p.1 - 1)) } else { None }
}
pub fn right(p: Point, _rows: usize, cols: usize) -> Option<Point> {
    (p.1 < cols - 1).then_some((p.0, p.1 + 1))
}
pub fn top_left(p: Point, _rows: usize, _cols: usize) -> Option<Point> {
    if p.0 > 0 && p.1 > 0 {
        Some((p.0 - 1, p.1 - 1))
    } else {
        None
    }
}
pub fn top_right(p: Point, _rows: usize, cols: usize) -> Option<Point> {
    if p.0 > 0 && p.1 < cols - 1 {
        Some((p.0 - 1, p.1 + 1))
    } else {
        None
    }
}
pub fn bottom_left(p: Point, rows: usize, _cols: usize) -> Option<Point> {
    if p.0 < rows - 1 && p.1 > 0 {
        Some((p.0 + 1, p.1 - 1))
    } else {
        None
    }
}
pub fn bottom_right(p: Point, rows: usize, cols: usize) -> Option<Point> {
    (p.0 < rows - 1 && p.1 < cols - 1).then_some((p.0 + 1, p.1 + 1))
}

pub fn adj4(p: Point, rows: usize, cols: usize) -> Adj4<Point> {
    Adj4 {
        top: top(p, rows, cols),
        bottom: bottom(p, rows, cols),
        left: left(p, rows, cols),
        right: right(p, rows, cols),
    }
}
pub fn adj8(p: Point, rows: usize, cols: usize) -> Adj8<Point> {
    Adj8 {
        top: top(p, rows, cols),
        bottom: bottom(p, rows, cols),
        left: left(p, rows, cols),
        right: right(p, rows, cols),
        top_left: top_left(p, rows, cols),
        top_right: top_right(p, rows, cols),
        bottom_left: bottom_left(p, rows, cols),
        bottom_right: bottom_right(p, rows, cols),
    }
}
