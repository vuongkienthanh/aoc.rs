pub mod u {
    use crate::adj::{Adj4, Adj8};
    pub type Point = (usize, usize);
    pub fn top(p: Point) -> Option<Point> {
        if p.0 > 0 {
            Some((p.0 - 1, p.1))
        } else {
            None
        }
    }
    pub fn bottom(p: Point) -> Option<Point> {
        Some((p.0 + 1, p.1))
    }
    pub fn left(p: Point) -> Option<Point> {
        if p.1 > 0 {
            Some((p.0, p.1 - 1))
        } else {
            None
        }
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
    pub fn adj4(p: Point) -> Adj4<Point> {
        Adj4 {
            top: top(p),
            bottom: bottom(p),
            left: left(p),
            right: right(p),
        }
    }
    pub fn adj8(p: Point) -> Adj8<Point> {
        Adj8 {
            top: top(p),
            bottom: bottom(p),
            left: left(p),
            right: right(p),
            top_left: top_left(p),
            top_right: top_right(p),
            bottom_left: bottom_left(p),
            bottom_right: bottom_right(p),
        }
    }
}
pub mod i {
    use crate::adj::{Adj4, Adj8};
    pub type Point = (isize, isize);
    pub fn top(p: Point) -> Option<Point> {
        Some((p.0 - 1, p.1))
    }
    pub fn bottom(p: Point) -> Option<Point> {
        Some((p.0 + 1, p.1))
    }
    pub fn left(p: Point) -> Option<Point> {
        Some((p.0, p.1 - 1))
    }
    pub fn right(p: Point) -> Option<Point> {
        Some((p.0, p.1 + 1))
    }
    pub fn top_left(p: Point) -> Option<Point> {
        Some((p.0 - 1, p.1 - 1))
    }
    pub fn top_right(p: Point) -> Option<Point> {
        Some((p.0 - 1, p.1 + 1))
    }
    pub fn bottom_left(p: Point) -> Option<Point> {
        Some((p.0 + 1, p.1 - 1))
    }
    pub fn bottom_right(p: Point) -> Option<Point> {
        Some((p.0 + 1, p.1 + 1))
    }
    pub fn adj4(p: Point) -> Adj4<Point> {
        Adj4 {
            top: top(p),
            bottom: bottom(p),
            left: left(p),
            right: right(p),
        }
    }
    pub fn adj8(p: Point) -> Adj8<Point> {
        Adj8 {
            top: top(p),
            bottom: bottom(p),
            left: left(p),
            right: right(p),
            top_left: top_left(p),
            top_right: top_right(p),
            bottom_left: bottom_left(p),
            bottom_right: bottom_right(p),
        }
    }
}
