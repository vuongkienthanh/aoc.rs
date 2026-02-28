// ####1#1#1#11#
// #89.0.1.2.34#
// ###1#3#5#7###
//   #0#2#4#6#
//   #########

use crate::{Cell, Map, State, run};
use std::ops::Range;

struct Map15;

impl Map for Map15 {
    fn room(&self, cell: &Cell) -> Range<usize> {
        match cell {
            Cell::A => 0..2,
            Cell::B => 2..4,
            Cell::C => 4..6,
            Cell::D => 6..8,
            Cell::E => panic!(),
        }
    }
    fn left(&self, cell: &Cell) -> impl Iterator<Item = usize> {
        match cell {
            Cell::A => (8..=9).rev(),
            Cell::B => (8..=10).rev(),
            Cell::C => (8..=11).rev(),
            Cell::D => (8..=12).rev(),
            Cell::E => panic!(),
        }
    }
    fn right(&self, cell: &Cell) -> impl Iterator<Item = usize> {
        match cell {
            Cell::A => 10..=14,
            Cell::B => 11..=14,
            Cell::C => 12..=14,
            Cell::D => 13..=14,
            Cell::E => panic!(),
        }
    }
    fn left_hall(&self) -> [usize; 2] {
        [9, 8]
    }
    fn right_hall(&self) -> [usize; 2] {
        [13, 14]
    }
    fn mid_hall(&self) -> [usize; 3] {
        [10, 11, 12]
    }
    fn step(&self, from: usize, to: usize) -> usize {
        match (from, to) {
            (0, 9) | (0, 10) | (2, 10) | (2, 11) | (4, 11) | (4, 12) | (6, 12) | (6, 13) => 3,
            (0, 11) | (2, 9) | (2, 12) | (4, 10) | (4, 13) | (6, 11) => 5,
            (0, 12) | (2, 13) | (4, 9) | (6, 10) => 7,
            (0, 13) | (6, 9) => 9,
            (x, 8) => self.step(x, 9) + 1,
            (x, 14) => self.step(x, 13) + 1,
            (1, y) => self.step(0, y) - 1,
            (3, y) => self.step(2, y) - 1,
            (5, y) => self.step(4, y) - 1,
            (7, y) => self.step(6, y) - 1,
            (x, y) if x >= 8 && y >= 8 => panic!("hall {x} to hall {y}"),
            (x, y) if x < 8 && y < 8 => panic!("room {x} to room {y}"),
            (x, y) => self.step(y, x),
        }
    }
    fn obstacle(&self, from: usize, to: usize) -> Vec<usize> {
        match (from, to) {
            (9, 0) | (10, 0) | (10, 2) | (11, 2) | (11, 4) | (12, 4) | (12, 6) | (13, 6) => vec![],
            (9, 2) | (11, 0) => vec![10],
            (9, 4) | (12, 0) => vec![10, 11],
            (9, 6) | (13, 0) => vec![10, 11, 12],
            (10, 4) | (12, 2) => vec![11],
            (10, 6) | (13, 2) => vec![11, 12],
            (11, 6) | (13, 4) => vec![12],
            (8, y) => self.obstacle(9, y),
            (14, y) => self.obstacle(13, y),
            (x, 1) => self.obstacle(x, 0),
            (x, 3) => self.obstacle(x, 2),
            (x, 5) => self.obstacle(x, 4),
            (x, 7) => self.obstacle(x, 6),
            (x, y) if x >= 8 && y >= 8 => panic!("hall {x} to hall {y}"),
            (x, y) if x < 8 && y < 8 => panic!("room {x} to room {y}"),
            (x, y) => panic!("room {x} to hall {y}"),
        }
    }
}

impl From<&str> for State<15> {
    fn from(value: &str) -> State<15> {
        let input = value
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut locations = [Cell::E; 15];
        for (r, c, i) in [
            (3, 3, 0),
            (2, 3, 1),
            (3, 5, 2),
            (2, 5, 3),
            (3, 7, 4),
            (2, 7, 5),
            (3, 9, 6),
            (2, 9, 7),
        ] {
            locations[i] = input[r][c].into();
        }
        State {
            locations,
            score: 0,
        }
    }
}

pub fn process(_input: &str) -> usize {
    run::<15>(State::from(_input), Map15)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 12521);
    }
}
