use crate::parsing::{Action, parse_input};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let instructions = parse_input(_input);
    let mut grid = Grid::<u8>::new(1000, 1000);
    for (action, (a0, a1), (b0, b1)) in instructions {
        match action {
            Action::On => {
                for r in a0..=b0 {
                    for i in grid.iter_row_mut(r).take(b1 + 1).skip(a1) {
                        *i += 1
                    }
                }
            }
            Action::Off => {
                for r in a0..=b0 {
                    for i in grid.iter_row_mut(r).take(b1 + 1).skip(a1) {
                        *i = i.saturating_sub(1)
                    }
                }
            }
            Action::Toggle => {
                for r in a0..=b0 {
                    for i in grid.iter_row_mut(r).take(b1 + 1).skip(a1) {
                        *i += 2
                    }
                }
            }
        }
    }
    grid.into_iter().map(|x| x as usize).sum()
}
