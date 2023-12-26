use crate::{Beam, Direction};
use std::collections::{HashMap, VecDeque};

pub fn process(_input: &str) -> usize {
    let max_rows = _input.lines().count();
    let max_cols = _input.lines().next().unwrap().len();
    let mut energized_tiles: HashMap<(usize, usize), Vec<Beam>> = HashMap::new();

    let mut beams = VecDeque::from([Beam {
        loc: (0, 0),
        direction: Direction::Right,
    }]);

    while let Some(beam) = beams.pop_front() {
        energized_tiles
            .entry(beam.loc)
            .or_default()
            .push(beam.clone());
        let point = _input
            .lines()
            .nth(beam.loc.0)
            .unwrap()
            .chars()
            .nth(beam.loc.1)
            .unwrap();
        let after_encounter = beam.encounter(point, max_rows, max_cols);
        for nxt_beam in after_encounter {
            if !energized_tiles
                .entry(nxt_beam.loc)
                .or_default()
                .contains(&nxt_beam)
            {
                energized_tiles
                    .get_mut(&nxt_beam.loc)
                    .unwrap()
                    .push(nxt_beam.clone());
                beams.push_back(nxt_beam);
            }
        }
    }

    energized_tiles.len()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(process(input), 46);
    }
}
