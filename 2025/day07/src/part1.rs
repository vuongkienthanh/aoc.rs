use crate::parsing::parse_input;
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let (start, splitters) = parse_input(_input);
    let mut ans = 0;
    let mut beams = vec![start];
    for mut splitter in splitters {
        let mut new_beams = HashSet::new();
        'a: for beam in beams {
            for s in 0..splitter.len() {
                if splitter[s] == beam {
                    new_beams.insert(beam - 1);
                    new_beams.insert(beam + 1);
                    ans += 1;
                    splitter.swap_remove(s);
                    continue 'a;
                }
            }
            new_beams.insert(beam);
        }
        beams = new_beams.into_iter().collect();
    }
    ans
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
        assert_eq!(process(fixture), 21);
    }
}
