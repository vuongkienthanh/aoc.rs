// #11#1#1#2#22#
// #67.8.9.0.12#
// ###3#7#1#5###
//   #2#6#0#4#
//   #1#5#9#3#
//   #0#4#8#2#
//   #########

use crate::{Cell, Map, State, run};
use std::ops::Range;

struct Map23;
impl Map for Map23 {
    fn room(&self, cell: &Cell) -> Range<usize> {
        match cell {
            Cell::A => 0..4,
            Cell::B => 4..8,
            Cell::C => 8..12,
            Cell::D => 12..16,
            Cell::E => panic!(),
        }
    }
    fn left(&self, cell: &Cell) -> impl Iterator<Item = usize> {
        match cell {
            Cell::A => (16..=17).rev(),
            Cell::B => (16..=18).rev(),
            Cell::C => (16..=19).rev(),
            Cell::D => (16..=20).rev(),
            Cell::E => panic!(),
        }
    }
    fn right(&self, cell: &Cell) -> impl Iterator<Item = usize> {
        match cell {
            Cell::A => 18..=22,
            Cell::B => 19..=22,
            Cell::C => 20..=22,
            Cell::D => 21..=22,
            Cell::E => panic!(),
        }
    }
    fn left_hall(&self) -> [usize; 2] {
        [17, 16]
    }
    fn right_hall(&self) -> [usize; 2] {
        [21, 22]
    }
    fn mid_hall(&self) -> [usize; 3] {
        [18, 19, 20]
    }
    fn step(&self, from: usize, to: usize) -> usize {
        match (from, to) {
            (0, 17) | (0, 18) | (4, 18) | (4, 19) | (8, 19) | (8, 20) | (12, 20) | (12, 21) => 5,
            (0, 19) | (4, 17) | (4, 20) | (8, 18) | (8, 21) | (12, 19) => 7,
            (0, 20) | (4, 21) | (8, 17) | (12, 18) => 9,
            (0, 21) | (12, 17) => 11,
            (x, 16) => self.step(x, 17) + 1,
            (x, 22) => self.step(x, 21) + 1,
            (1..=3, y) => self.step(0, y) - from,
            (5..=7, y) => self.step(4, y) + 4 - from,
            (9..=11, y) => self.step(8, y) + 8 - from,
            (13..=15, y) => self.step(12, y) + 12 - from,
            (x, y) if x >= 16 && y >= 16 => panic!("hall {x} to hall {y}"),
            (x, y) if x < 16 && y < 16 => panic!("room {x} to room {y}"),
            (x, y) => self.step(y, x),
        }
    }
    fn obstacle(&self, from: usize, to: usize) -> Vec<usize> {
        match (from, to) {
            (17, 0) | (18, 0) | (18, 4) | (19, 4) | (19, 8) | (20, 8) | (20, 12) | (21, 12) => {
                vec![]
            }
            (17, 4) | (19, 0) => vec![18],
            (17, 8) | (20, 0) => vec![18, 19],
            (17, 12) | (21, 0) => vec![18, 19, 20],
            (18, 8) | (20, 4) => vec![19],
            (18, 12) | (21, 4) => vec![19, 20],
            (19, 12) | (21, 8) => vec![20],
            (16, y) => self.obstacle(17, y),
            (22, y) => self.obstacle(21, y),
            (x, 1..=3) => self.obstacle(x, 0),
            (x, 5..=7) => self.obstacle(x, 4),
            (x, 9..=11) => self.obstacle(x, 8),
            (x, 13..=15) => self.obstacle(x, 12),
            (x, y) if x >= 16 && y >= 16 => panic!("hall {x} to hall {y}"),
            (x, y) if x < 16 && y < 16 => panic!("room {x} to room {y}"),
            (x, y) => panic!("room {x} to hall {y}"),
        }
    }
}
impl From<&str> for State<23> {
    fn from(value: &str) -> State<23> {
        let input = value
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut locations = [Cell::E; 23];
        for (r, c, i) in [
            (3, 3, 0),
            (2, 3, 3),
            (3, 5, 4),
            (2, 5, 7),
            (3, 7, 8),
            (2, 7, 11),
            (3, 9, 12),
            (2, 9, 15),
        ] {
            locations[i] = input[r][c].into();
        }
        for (i, cell) in [
            (1, Cell::D),
            (2, Cell::D),
            (5, Cell::B),
            (6, Cell::C),
            (9, Cell::A),
            (10, Cell::B),
            (13, Cell::C),
            (14, Cell::A),
        ] {
            locations[i] = cell;
        }
        State {
            locations,
            score: 0,
        }
    }
}

pub fn process(_input: &str) -> usize {
    run::<23>(State::from(_input), Map23)
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
        assert_eq!(process(fixture), 44169);
    }
}
