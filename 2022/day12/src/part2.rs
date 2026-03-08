use crate::parsing::{Point, parse_input};
use aoc_helper::adj::grid::adj4;
use fxhash::FxHashSet;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (grid, _, end) = parse_input(_input);
    path(&grid, end)
}

fn path(grid: &Grid<u8>, end: Point) -> usize {
    let mut seen: FxHashSet<Point> = FxHashSet::default();
    seen.insert(end);
    let mut step = 0;
    let mut current = vec![end];

    'ans: loop {
        let mut new = vec![];
        for curr in current {
            if grid[curr] == b'a' {
                break 'ans step;
            }
            for adj in adj4(curr, grid.rows(), grid.cols()).into_iter().flatten() {
                if (grid[curr] - 1..=b'z').contains(&grid[adj]) && seen.insert(adj) {
                    new.push(adj);
                }
            }
        }
        step += 1;
        current = new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 29);
    }
}
