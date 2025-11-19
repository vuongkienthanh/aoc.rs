use crate::parsing::{Item, parse_input};
use grid::Grid;

pub fn process(_input: &str) -> String {
    let (_, input) = parse_input(_input).unwrap();

    let grid = Grid::from(vec![
        vec!['*', '*', '1', '*', '*'],
        vec!['*', '2', '3', '4', '*'],
        vec!['5', '6', '7', '8', '9'],
        vec!['*', 'A', 'B', 'C', '*'],
        vec!['*', '*', 'D', '*', '*'],
    ]);

    let mut coord = (2, 0);
    let mut ans = String::new();
    for line in input {
        for i in line {
            match i {
                Item::U => coord = up(coord, &grid),
                Item::D => coord = down(coord, &grid),
                Item::L => coord = left(coord, &grid),
                Item::R => coord = right(coord, &grid),
            }
        }
        ans.push(grid[coord]);
    }

    ans
}

fn up(c: (usize, usize), grid: &Grid<char>) -> (usize, usize) {
    if c.0 == 0 || grid[(c.0 - 1, c.1)] == '*' {
        c
    } else {
        (c.0 - 1, c.1)
    }
}
fn down(c: (usize, usize), grid: &Grid<char>) -> (usize, usize) {
    if c.0 == grid.rows() - 1 || grid[(c.0 + 1, c.1)] == '*' {
        c
    } else {
        (c.0 + 1, c.1)
    }
}
fn left(c: (usize, usize), grid: &Grid<char>) -> (usize, usize) {
    if c.1 == 0 || grid[(c.0, c.1 - 1)] == '*' {
        c
    } else {
        (c.0, c.1 - 1)
    }
}
fn right(c: (usize, usize), grid: &Grid<char>) -> (usize, usize) {
    if c.1 == grid.cols() - 1 || grid[(c.0, c.1 + 1)] == '*' {
        c
    } else {
        (c.0, c.1 + 1)
    }
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
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), "5DB3");
    }
}
