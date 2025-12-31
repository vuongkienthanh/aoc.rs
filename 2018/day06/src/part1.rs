use crate::parsing::parse_input;
use crate::{get_rows_cols, normalize};
use aoc_helper::adj::grid::adj4;

type Point = (usize, usize);
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd)]
enum Cell {
    #[default]
    Blank,
    Conflict,
    Own(usize),
    Expanding(usize),
}

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let input = normalize(input);
    let (rows, cols) = get_rows_cols(&input);
    let mut grid = build_grid_with(rows, cols, &input);
    let mut is_infinite = vec![false; input.len()];
    let mut state: Vec<Vec<Point>> = input.iter().map(|x| vec![*x]).collect();

    while state.iter().any(|v| !v.is_empty()) {
        let mut new_state: Vec<Vec<Point>> = vec![];
        for (i, group) in state.into_iter().enumerate() {
            let mut new_group: Vec<Point> = vec![];
            for p in group {
                let adjs = adj4(p, rows, cols);
                // touch wall
                if adjs.iter().any(|x| x.is_none()) {
                    is_infinite[i] = true;
                }
                for adj in adjs.into_iter().flatten() {
                    match grid[adj.0][adj.1] {
                        Cell::Blank => {
                            grid[adj.0][adj.1] = Cell::Expanding(i);
                            new_group.push(adj);
                        }
                        Cell::Expanding(x) if x != i => {
                            grid[adj.0][adj.1] = Cell::Conflict;
                        }
                        _ => (),
                    }
                }
            }
            new_state.push(new_group);
        }

        // finalize cells from expanding to owned
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                if let Cell::Expanding(x) = cell {
                    *cell = Cell::Own(*x);
                }
            }
        }

        // remove conflicted cells
        for group in new_state.iter_mut() {
            group.retain(|p| grid[p.0][p.1] != Cell::Conflict)
        }

        state = new_state;
    }
    grid.into_iter()
        .flat_map(|line| line.into_iter())
        .fold(vec![0; input.len()], |mut acc, ele| {
            if let Cell::Own(x) = ele {
                acc[x] += 1;
            }
            acc
        })
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| (!is_infinite[i]).then_some(v))
        .max()
        .unwrap()
}

fn build_grid_with(rows: usize, cols: usize, input: &[Point]) -> Vec<Vec<Cell>> {
    let mut grid = vec![vec![Cell::Blank; cols]; rows];
    for (i, p) in input.iter().enumerate() {
        grid[p.0][p.1] = Cell::Own(i);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 17);
    }
}
