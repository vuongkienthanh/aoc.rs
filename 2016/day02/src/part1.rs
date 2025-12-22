use crate::parsing::{Item, parse_input};

const ROWS: usize = 3;
const COLS: usize = 3;
const GRID: [[usize; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut coord = (1, 1);
    let mut ans = 0;

    for line in input {
        for i in line {
            match i {
                Item::U => coord = up(coord),
                Item::D => coord = down(coord),
                Item::L => coord = left(coord),
                Item::R => coord = right(coord),
            }
        }
        ans = ans * 10 + GRID[coord.0][coord.1];
    }

    ans
}

fn up(c: (usize, usize)) -> (usize, usize) {
    if c.0 == 0 { c } else { (c.0 - 1, c.1) }
}
fn down(c: (usize, usize)) -> (usize, usize) {
    if c.0 == ROWS - 1 { c } else { (c.0 + 1, c.1) }
}
fn left(c: (usize, usize)) -> (usize, usize) {
    if c.1 == 0 { c } else { (c.0, c.1 - 1) }
}
fn right(c: (usize, usize)) -> (usize, usize) {
    if c.1 == COLS - 1 { c } else { (c.0, c.1 + 1) }
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
