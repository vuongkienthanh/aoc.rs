use crate::parsing::{Action, parse_input};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let instructions = parse_input(_input);
    let mut grid = Grid::<usize>::new(1000, 1000);
    for (action, top_left, bottom_right) in instructions {
        match action {
            Action::On => {
                for row in (top_left.0)..=(bottom_right.0) {
                    for col in (top_left.1)..=(bottom_right.1) {
                        grid[(row, col)] = 1
                    }
                }
            }
            Action::Off => {
                for row in (top_left.0)..=(bottom_right.0) {
                    for col in (top_left.1)..=(bottom_right.1) {
                        grid[(row, col)] = 0
                    }
                }
            }
            Action::Toggle => {
                for row in (top_left.0)..=(bottom_right.0) {
                    for col in (top_left.1)..=(bottom_right.1) {
                        grid[(row, col)] = 1 - grid[(row, col)]
                    }
                }
            }
        }
    }
    grid.into_iter().sum()
}
