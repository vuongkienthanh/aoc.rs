pub mod part1;
pub mod part2;

use grid::Grid;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
    step: u8,
    direction: Direction,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

impl Point {
    fn new(x: usize, y: usize, step: u8, direction: Direction) -> Self {
        Self {
            x,
            y,
            step,
            direction,
        }
    }

    #[rustfmt::skip]
    fn left(&self, grid: &Grid<usize>) -> Option<Point> {
        match self.direction {
            Direction::Up => { if self.y == 0 { None } else { Some(Point::new(self.x, self.y - 1, 1, Direction::Left)) } }
            Direction::Down => { if self.y == grid.cols() - 1 { None } else { Some(Point::new(self.x, self.y + 1, 1, Direction::Right)) } }
            Direction::Left => { if self.x == grid.rows() - 1 { None } else { Some(Point::new(self.x + 1, self.y, 1, Direction::Down)) } }
            Direction::Right => { if self.x == 0 { None } else { Some(Point::new(self.x - 1, self.y, 1, Direction::Up)) } }
        }
    }
    #[rustfmt::skip]
    fn right(&self, grid: &Grid<usize>) -> Option<Point> {
        match self.direction {
            Direction::Up => { if self.y == grid.cols() - 1 { None } else { Some(Point::new(self.x, self.y + 1, 1, Direction::Right)) } }
            Direction::Down => { if self.y == 0 { None } else { Some(Point::new(self.x, self.y - 1, 1, Direction::Left)) } }
            Direction::Left => { if self.x == 0 { None } else { Some(Point::new(self.x - 1, self.y, 1, Direction::Up)) } }
            Direction::Right => { if self.x == grid.rows() - 1 { None } else { Some(Point::new(self.x + 1, self.y, 1, Direction::Down)) } }
        }
    }
    #[rustfmt::skip]
    fn straight(&self, grid: &Grid<usize>) -> Option<Point> {
        match self.direction {
            Direction::Up => { if self.x == 0 { None } else { Some(Point::new(self.x - 1, self.y, self.step + 1, self.direction.clone())) } }
            Direction::Down => { if self.x == grid.rows() - 1 { None } else { Some(Point::new(self.x + 1, self.y, self.step + 1, self.direction.clone())) } }
            Direction::Left => { if self.y == 0 { None } else { Some(Point::new(self.x, self.y - 1, self.step + 1, self.direction.clone())) } }
            Direction::Right => { if self.y == grid.cols() - 1 { None } else { Some(Point::new(self.x, self.y + 1, self.step + 1, self.direction.clone())) } }
        }
    }
}
