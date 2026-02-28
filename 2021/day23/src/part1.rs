// ####1#1#1#11#
// #89.0.1.2.34#
// ###1#3#5#7###
//   #0#2#4#6#
//   #########

use crate::{Cell, State, run};

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
    run::<15>(State::from(_input))
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
