use crate::parsing::{Point, parse_input};
use aoc_helper::adj::grid::adj4;
use fxhash::FxHashSet;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (grid, start, end) = parse_input(_input);
    path(&grid, start, end)
}

fn path(grid: &Grid<u8>, start: Point, end: Point) -> usize {
    let mut seen: FxHashSet<Point> = FxHashSet::default();
    seen.insert(start);
    let mut step = 0;
    let mut current = vec![start];

    'ans: loop {
        let mut new = vec![];
        for curr in current {
            if curr == end {
                break 'ans step;
            }
            for adj in adj4(curr, grid.rows(), grid.cols()).into_iter().flatten() {
                if (b'a'..=grid[curr] + 1).contains(&grid[adj]) && seen.insert(adj) {
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
        assert_eq!(process(fixture), 31);
    }
}
