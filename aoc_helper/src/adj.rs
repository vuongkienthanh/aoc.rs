//! `adj4`, `adj8` to get adjacent of a point in a Coordinate System
//!
//! provide 3 flavors for adj functions:
//! - `naive`: to work with usize or isize without check, may panic.
//! - `grid`: to work in grid based puzzle, add checks to grid borders.
//! - `checked_u`: to work with usize but add checks.

pub mod grid {
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

    pub fn adj4(p: Point, rows: usize, cols: usize) -> [Option<Point>; 4] {
        [
            top(p, rows, cols),
            bottom(p, rows, cols),
            left(p, rows, cols),
            right(p, rows, cols),
        ]
    }
    pub fn adj8(p: Point, rows: usize, cols: usize) -> [Option<Point>; 8] {
        [
            top(p, rows, cols),
            bottom(p, rows, cols),
            left(p, rows, cols),
            right(p, rows, cols),
            top_left(p, rows, cols),
            top_right(p, rows, cols),
            bottom_left(p, rows, cols),
            bottom_right(p, rows, cols),
        ]
    }
}
pub mod checked_u {
    pub type Point = (usize, usize);
    pub fn top(p: Point) -> Option<Point> {
        if p.0 > 0 { Some((p.0 - 1, p.1)) } else { None }
    }
    pub fn bottom(p: Point) -> Option<Point> {
        Some((p.0 + 1, p.1))
    }
    pub fn left(p: Point) -> Option<Point> {
        if p.1 > 0 { Some((p.0, p.1 - 1)) } else { None }
    }
    pub fn right(p: Point) -> Option<Point> {
        Some((p.0, p.1 + 1))
    }
    pub fn top_left(p: Point) -> Option<Point> {
        if p.0 > 0 && p.1 > 0 {
            Some((p.0 - 1, p.1 - 1))
        } else {
            None
        }
    }
    pub fn top_right(p: Point) -> Option<Point> {
        if p.0 > 0 {
            Some((p.0 - 1, p.1 + 1))
        } else {
            None
        }
    }
    pub fn bottom_left(p: Point) -> Option<Point> {
        if p.1 > 0 {
            Some((p.0 + 1, p.1 - 1))
        } else {
            None
        }
    }
    pub fn bottom_right(p: Point) -> Option<Point> {
        Some((p.0 + 1, p.1 + 1))
    }
    pub fn adj4(p: Point) -> [Option<Point>; 4] {
        [top(p), bottom(p), left(p), right(p)]
    }
    pub fn adj8(p: Point) -> [Option<Point>; 8] {
        [
            top(p),
            bottom(p),
            left(p),
            right(p),
            top_left(p),
            top_right(p),
            bottom_left(p),
            bottom_right(p),
        ]
    }
}

pub mod naive {
    use num_traits::{One, PrimInt};
    pub fn top<T>(p: (T, T)) -> (T, T)
    where
        T: PrimInt + One,
    {
        (p.0 - One::one(), p.1)
    }
    pub fn bottom<T>(p: (T, T)) -> (T, T)
    where
        T: PrimInt + One,
    {
        (p.0 + One::one(), p.1)
    }
    pub fn left<T>(p: (T, T)) -> (T, T)
    where
        T: PrimInt + One,
    {
        (p.0, p.1 - One::one())
    }
    pub fn right<T>(p: (T, T)) -> (T, T)
    where
        T: PrimInt + One,
    {
        (p.0, p.1 + One::one())
    }
    pub fn top_left<T>(p: (T, T)) -> (T, T)
    where
        T: PrimInt + One,
    {
        (p.0 - One::one(), p.1 - One::one())
    }
    pub fn top_right<T>(p: (T, T)) -> (T, T)
    where
        T: PrimInt + One,
    {
        (p.0 - One::one(), p.1 + One::one())
    }
    pub fn bottom_left<T>(p: (T, T)) -> (T, T)
    where
        T: PrimInt + One,
    {
        (p.0 + One::one(), p.1 - One::one())
    }
    pub fn bottom_right<T>(p: (T, T)) -> (T, T)
    where
        T: PrimInt + One,
    {
        (p.0 + One::one(), p.1 + One::one())
    }
    pub fn adj4<T>(p: (T, T)) -> [(T, T); 4]
    where
        T: PrimInt + One,
    {
        [top(p), bottom(p), left(p), right(p)]
    }
    pub fn adj8<T>(p: (T, T)) -> [(T, T); 8]
    where
        T: PrimInt + One,
    {
        [
            top(p),
            bottom(p),
            left(p),
            right(p),
            top_left(p),
            top_right(p),
            bottom_left(p),
            bottom_right(p),
        ]
    }
}
