use crate::parsing::{Item, parse_input};

const ROWS: usize = 5;
const COLS: usize = 5;
const GRID: [[char; 5]; 5] = [
    ['*', '*', '1', '*', '*'],
    ['*', '2', '3', '4', '*'],
    ['5', '6', '7', '8', '9'],
    ['*', 'A', 'B', 'C', '*'],
    ['*', '*', 'D', '*', '*'],
];

pub fn process(_input: &str) -> String {
    let input = parse_input(_input);

    let mut coord = (2, 0);
    let mut ans = String::new();
    for line in input {
        for i in line {
            match i {
                Item::U => coord = up(coord),
                Item::D => coord = down(coord),
                Item::L => coord = left(coord),
                Item::R => coord = right(coord),
            }
        }
        ans.push(GRID[coord.0][coord.1]);
    }

    ans
}

fn up(c: (usize, usize)) -> (usize, usize) {
    if c.0 == 0 || GRID[c.0 - 1][c.1] == '*' {
        c
    } else {
        (c.0 - 1, c.1)
    }
}
fn down(c: (usize, usize)) -> (usize, usize) {
    if c.0 == ROWS - 1 || GRID[c.0 + 1][c.1] == '*' {
        c
    } else {
        (c.0 + 1, c.1)
    }
}
fn left(c: (usize, usize)) -> (usize, usize) {
    if c.1 == 0 || GRID[c.0][c.1 - 1] == '*' {
        c
    } else {
        (c.0, c.1 - 1)
    }
}
fn right(c: (usize, usize)) -> (usize, usize) {
    if c.1 == COLS - 1 || GRID[c.0][c.1 + 1] == '*' {
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
