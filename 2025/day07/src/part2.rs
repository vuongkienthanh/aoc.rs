use crate::parsing::parse_input;
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let (start, splitters) = parse_input(_input);
    let mut beams = BTreeMap::new();
    beams.insert(start, 1);
    for mut splitter in splitters {
        let mut new_beams = BTreeMap::new();
        'a: for (beam, v) in beams {
            for s in 0..splitter.len() {
                if splitter[s] == beam {
                    *new_beams.entry(beam - 1).or_default() += v;
                    *new_beams.entry(beam + 1).or_default() += v;
                    splitter.swap_remove(s);
                    continue 'a;
                }
            }
            *new_beams.entry(beam).or_default() += v;
        }
        beams = new_beams;
    }
    beams.into_values().sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 40);
    }
}
