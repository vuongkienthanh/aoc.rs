use crate::{Action, parse_input};
use grid::Grid;
pub fn process(_input: &str) -> usize {
    let (_, instructions) = parse_input(_input).expect("parse succeed");
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
#[cfg(test)]
mod tests {
    // use super::*;
    use crate::{Action, Coord, parse_line};
    use rstest::rstest;
    #[rstest]
    #[case("turn on 0,0 through 999,999", (Action::On, (0,0), (999,999)))]
    #[case("toggle 0,0 through 999,0", (Action::Toggle, (0,0), (999,0)))]
    #[case("turn off 499,499 through 500,500", (Action::Off, (499,499), (500,500)))]
    fn test_parse_line(#[case] input: &str, #[case] expected: (Action, Coord, Coord)) {
        let (_, line) = parse_line(input).expect("parse succeed");
        assert_eq!(line, expected);
    }
}
