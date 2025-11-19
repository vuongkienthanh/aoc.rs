pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Coord = (isize, isize);

impl Direction {
    pub fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
    pub fn turn_left(&self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }
    pub fn forward(&self, c: Coord, n: isize) -> Coord {
        use Direction::*;
        match self {
            Up => (c.0 - n, c.1),
            Down => (c.0 + n, c.1),
            Left => (c.0, c.1 - n),
            Right => (c.0, c.1 + n),
        }
    }
}
