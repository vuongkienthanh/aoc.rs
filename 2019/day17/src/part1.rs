use aoc_helper::adj::checked_u::adj4;
use intcode::{Computer, RunResult, parse};
use std::collections::BTreeSet;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut camera = Computer::new(input);
    let mut seen = BTreeSet::new();
    let mut loc = (0, 0);
    loop {
        let output = if let RunResult::Output(output) = camera.long_run() {
            output as u8 as char
        } else {
            break;
        };
        match output {
            '.' => {
                loc = (loc.0 + 1, loc.1);
            }
            '#' | '^' | 'v' | '>' | '<' => {
                seen.insert(loc);
                loc = (loc.0 + 1, loc.1);
            }
            '\n' => {
                loc = (0, loc.1 + 1);
            }
            _ => panic!(),
        }
        print!("{}", output);
    }
    seen.iter()
        .filter(|p| adj4(**p).into_iter().flatten().all(|p2| seen.contains(&p2)))
        .map(|(a, b)| a * b)
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#""#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 0);
    }

    #[rstest]
    #[case("", 0)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
