//! Directions with some methods

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

pub fn step(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

pub fn many_steps(dir: Direction, count: usize) -> Vec<(isize, isize)> {
    use std::iter::repeat_n;
    match dir {
        Direction::Up => repeat_n(-1, count).map(|x| (x, 0)).collect(),
        Direction::Down => repeat_n(1, count).map(|x| (x, 0)).collect(),
        Direction::Left => repeat_n(-1, count).map(|y| (0, y)).collect(),
        Direction::Right => repeat_n(1, count).map(|y| (0, y)).collect(),
    }
}

pub fn jump(dir: Direction, count: usize) -> (isize, isize) {
    let count = count as isize;
    match dir {
        Direction::Up => (-count, 0),
        Direction::Down => (count, 0),
        Direction::Left => (0, -count),
        Direction::Right => (0, count),
    }
}
