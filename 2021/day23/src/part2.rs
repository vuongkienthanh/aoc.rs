// #11#1#1#2#22#
// #67.8.9.0.12#
// ###3#7#1#5###
//   #2#6#0#4#
//   #1#5#9#3#
//   #0#4#8#2#
//   #########

use crate::{Cell, State, run};

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
    run::<23>(State::from(_input))
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
