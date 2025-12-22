use crate::parsing::parse_grid;
use aoc_helper::adj::grid::adj8;
use grid::Grid;
pub fn process(_input: &str) -> usize {
    count(steps(parse_grid(_input), 100))
}
fn turn_on_corner(mut grid: Grid<usize>) -> Grid<usize> {
    let rows = grid.rows();
    let cols = grid.cols();
    grid[(0, 0)] = 1;
    grid[(0, cols - 1)] = 1;
    grid[(rows - 1, 0)] = 1;
    grid[(rows - 1, cols - 1)] = 1;
    grid
}

fn steps(mut grid: Grid<usize>, n: usize) -> Grid<usize> {
    let rows = grid.rows();
    let cols = grid.cols();
    for _ in 0..n {
        grid = turn_on_corner(grid);
        let mut new_grid = grid.clone();
        for (coord, ele) in grid.indexed_iter() {
            match ele {
                1 => {
                    let adj = adj8(coord, rows, cols)
                        .into_iter()
                        .filter(|c| c.is_some_and(|x| grid[x] == 1))
                        .count();
                    if adj != 2 && adj != 3 {
                        new_grid[coord] = 0;
                    }
                }
                0 => {
                    let adj = adj8(coord, rows, cols)
                        .into_iter()
                        .filter(|c| c.is_some_and(|x| grid[x] == 1))
                        .count();
                    if adj == 3 {
                        new_grid[coord] = 1;
                    }
                }
                _ => (),
            }
        }
        grid = new_grid;
    }
    turn_on_corner(grid)
}
fn count(grid: Grid<usize>) -> usize {
    grid.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let grid = parse_grid(
            r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#,
        );
        assert_eq!(count(steps(grid, 5)), 17);
    }
}
