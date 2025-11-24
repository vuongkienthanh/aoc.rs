#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
}
