use grid::Grid;
use pathfinding::prelude::astar;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    step: u8,
    direction: Direction,
}
// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }
// impl Eq for Point {}

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
    fn left(&self, grid: &Grid<u32>) -> Option<Point> {
        match self.direction {
            Direction::Up => { if self.y == 0 { None } else { Some(Point::new(self.x, self.y - 1, 0, Direction::Left)) } }
            Direction::Down => { if self.y == grid.cols() - 1 { None } else { Some(Point::new(self.x, self.y + 1, 0, Direction::Right)) } }
            Direction::Left => { if self.x == grid.rows() - 1 { None } else { Some(Point::new(self.x + 1, self.y, 0, Direction::Down)) } }
            Direction::Right => { if self.x == 0 { None } else { Some(Point::new(self.x - 1, self.y, 0, Direction::Up)) } }
        }
    }
    #[rustfmt::skip]
    fn right(&self, grid: &Grid<u32>) -> Option<Point> {
        match self.direction {
            Direction::Up => { if self.y == grid.cols() - 1 { None } else { Some(Point::new(self.x, self.y + 1, 0, Direction::Right)) } }
            Direction::Down => { if self.y == 0 { None } else { Some(Point::new(self.x, self.y - 1, 0, Direction::Left)) } }
            Direction::Left => { if self.x == 0 { None } else { Some(Point::new(self.x - 1, self.y, 0, Direction::Up)) } }
            Direction::Right => { if self.x == grid.rows() - 1 { None } else { Some(Point::new(self.x + 1, self.y, 0, Direction::Down)) } }
        }
    }
    #[rustfmt::skip]
    fn straight(&self, grid: &Grid<u32>) -> Option<Point> {
        if self.step == 3 {
            return None
        }
        match self.direction {
            Direction::Up => { if self.x == 0 { None } else { Some(Point::new(self.x - 1, self.y, self.step + 1, self.direction.clone())) } }
            Direction::Down => { if self.x == grid.rows() - 1 { None } else { Some(Point::new(self.x + 1, self.y, self.step + 1, self.direction.clone())) } }
            Direction::Left => { if self.y == 0 { None } else { Some(Point::new(self.x, self.y - 1, self.step + 1, self.direction.clone())) } }
            Direction::Right => { if self.y == grid.cols() - 1 { None } else { Some(Point::new(self.x, self.y + 1, self.step + 1, self.direction.clone())) } }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn process(input: &str) -> usize {
    let grid = Grid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let start = Point::new(0, 0, 0, Direction::Right);

    dbg!(astar(
        &start,
        |point| {
            [point.straight(&grid), point.left(&grid), point.right(&grid)]
                .into_iter()
                .filter_map(|somepoint| somepoint)
                .map(|p| (p.clone(), *grid.get(p.x, p.y).unwrap()))
        },
        |point| ((grid.rows() - 1).abs_diff(point.x) + (grid.cols() - 1).abs_diff(point.y)) as u32,
        |point| point.x == grid.rows() - 1 && point.y == grid.cols() - 1,
    )
    .unwrap());
        0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
        assert_eq!(process(input), 102);
    }
}
