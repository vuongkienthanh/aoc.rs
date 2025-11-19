use crate::parsing::{Item, parse_input};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());
    // println!("{input:?}");

    let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    let mut coord = (1, 1);
    let mut ans = 0;

    for line in input {
        for i in line {
            match i {
                Item::U => coord = up(coord, grid.rows(), grid.cols()),
                Item::D => coord = down(coord, grid.rows(), grid.cols()),
                Item::L => coord = left(coord, grid.rows(), grid.cols()),
                Item::R => coord = right(coord, grid.rows(), grid.cols()),
            }
        }
        ans = ans * 10 + grid[coord];
    }

    ans
}

fn up(c: (usize, usize), _rows: usize, _cols: usize) -> (usize, usize) {
    if c.0 == 0 { c } else { (c.0 - 1, c.1) }
}
fn down(c: (usize, usize), rows: usize, _cols: usize) -> (usize, usize) {
    if c.0 == rows - 1 { c } else { (c.0 + 1, c.1) }
}
fn left(c: (usize, usize), _rows: usize, _cols: usize) -> (usize, usize) {
    if c.1 == 0 { c } else { (c.0, c.1 - 1) }
}
fn right(c: (usize, usize), _rows: usize, cols: usize) -> (usize, usize) {
    if c.1 == cols - 1 { c } else { (c.0, c.1 + 1) }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"ULL
RRDDD
LURDL
UUUUD"#
    }
    #[rstest]
    fn test_process_1(fixture: &str) {
        assert_eq!(process(fixture), 1985);
    }
}
