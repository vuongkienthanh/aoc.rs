use crate::parsing::{Item, parse_input};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let mut input = Grid::from(parse_input(_input));
    loop {
        let mut new_input = input.clone();
        for ((i, j), cell) in new_input.indexed_iter_mut() {
            match cell {
                Item::Space => (),
                Item::Empty => {
                    if adj8((i, j), &input)
                        .into_iter()
                        .flatten()
                        .filter(|(r, c)| matches!(input[(*r, *c)], Item::Occupied))
                        .count()
                        == 0
                    {
                        *cell = Item::Occupied;
                    }
                }
                Item::Occupied => {
                    if adj8((i, j), &input)
                        .into_iter()
                        .flatten()
                        .filter(|(r, c)| matches!(input[(*r, *c)], Item::Occupied))
                        .count()
                        >= 5
                    {
                        *cell = Item::Empty;
                    }
                }
            }
        }
        if new_input == input {
            break new_input
                .into_iter()
                .filter(|x| matches!(x, Item::Occupied))
                .count();
        } else {
            input = new_input
        }
    }
}

fn adj8((row, col): (usize, usize), grid: &Grid<Item>) -> [Option<(usize, usize)>; 8] {
    let top = (0..row)
        .rfind(|row| matches!(grid[(*row, col)], Item::Occupied | Item::Empty))
        .map(|row| (row, col));
    let bottom = (row + 1..grid.rows())
        .find(|row| matches!(grid[(*row, col)], Item::Occupied | Item::Empty))
        .map(|row| (row, col));
    let left = (0..col)
        .rfind(|col| matches!(grid[(row, *col)], Item::Occupied | Item::Empty))
        .map(|col| (row, col));
    let right = (col + 1..grid.cols())
        .find(|col| matches!(grid[(row, *col)], Item::Occupied | Item::Empty))
        .map(|col| (row, col));
    let top_left = ((0..row).rev())
        .zip((0..col).rev())
        .find(|(row, col)| matches!(grid[(*row, *col)], Item::Occupied | Item::Empty));
    let top_right = ((0..row).rev())
        .zip(col + 1..grid.cols())
        .find(|(row, col)| matches!(grid[(*row, *col)], Item::Occupied | Item::Empty));
    let bottom_left = (row + 1..grid.rows())
        .zip((0..col).rev())
        .find(|(row, col)| matches!(grid[(*row, *col)], Item::Occupied | Item::Empty));
    let bottom_right = (row + 1..grid.rows())
        .zip(col + 1..grid.cols())
        .find(|(row, col)| matches!(grid[(*row, *col)], Item::Occupied | Item::Empty));
    [
        top,
        bottom,
        left,
        right,
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    ]
}
