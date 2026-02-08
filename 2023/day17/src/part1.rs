use crate::{Direction, Point};
use grid::Grid;
use pathfinding::prelude::astar;

impl Point {
    fn left1(&self, grid: &Grid<usize>) -> Option<Point> {
        self.left(grid)
    }
    fn right1(&self, grid: &Grid<usize>) -> Option<Point> {
        self.right(grid)
    }
    fn straight1(&self, grid: &Grid<usize>) -> Option<Point> {
        if self.step == 3 {
            return None;
        }
        self.straight(grid)
    }
}

pub fn process(input: &str) -> usize {
    let grid = Grid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let start = Point::new(0, 0, 0, Direction::Right);
    let end_x = grid.rows() - 1;
    let end_y = grid.cols() - 1;

    astar(
        &start,
        |point| {
            [
                point.straight1(&grid),
                point.left1(&grid),
                point.right1(&grid),
            ]
            .into_iter()
            .filter_map(|somepoint| somepoint)
            .map(|p| (p.clone(), *grid.get(p.x, p.y).unwrap()))
        },
        |point| (end_x.abs_diff(point.x) + end_y.abs_diff(point.y)),
        |point| point.x == end_x && point.y == end_y,
    )
    .unwrap()
    .1
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
