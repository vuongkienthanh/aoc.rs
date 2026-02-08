use crate::{Direction, Point};
use grid::Grid;
use pathfinding::prelude::astar;

impl Point {
    fn left2(&self, grid: &Grid<usize>) -> Option<Point> {
        // ignore start and min 4
        if (1..4).contains(&self.step) {
            return None;
        }
        self.left(grid)
    }
    fn right2(&self, grid: &Grid<usize>) -> Option<Point> {
        // ignore start and min 4
        if (1..4).contains(&self.step) {
            return None;
        }
        self.right(grid)
    }
    fn straight2(&self, grid: &Grid<usize>) -> Option<Point> {
        // max of 10
        if self.step == 10 {
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
        // successor function (next point, score)
        |point| {
            [
                point.straight2(&grid),
                point.left2(&grid),
                point.right2(&grid),
            ]
            .into_iter()
            .filter_map(|somepoint| somepoint)
            .map(|p| (p.clone(), *grid.get(p.x, p.y).unwrap()))
        },
        // heuristic i.e. manhattan
        |point| (end_x.abs_diff(point.x) + end_y.abs_diff(point.y)),
        // end condition
        // step must >= 4 before end
        |point| point.x == end_x && point.y == end_y && point.step >= 4,
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
        assert_eq!(process(input), 94);
    }
    #[test]
    fn test_process_2() {
        let input = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;
        assert_eq!(process(input), 71);
    }
}
