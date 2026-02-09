use crate::parsing::{Item, parse_input};
use aoc_helper::adj::grid::adj8;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let mut input = Grid::from(parse_input(_input));

    loop {
        let mut new_input = input.clone();
        for ((i, j), cell) in new_input.indexed_iter_mut() {
            match cell {
                Item::Space => (),
                Item::Empty => {
                    if adj8((i, j), input.rows(), input.cols())
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
                    if adj8((i, j), input.rows(), input.cols())
                        .into_iter()
                        .flatten()
                        .filter(|(r, c)| matches!(input[(*r, *c)], Item::Occupied))
                        .count()
                        >= 4
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
