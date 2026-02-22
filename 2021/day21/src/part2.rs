use crate::parsing::parse_input;
use fxhash::FxHashMap;

pub fn process(_input: &str) -> u64 {
    let (p1, p2) = parse_input(_input);
    let mut win = [0u64, 0u64];
    let mut current = FxHashMap::default();
    current.insert([(p1 - 1, 0), (p2 - 1, 0)], 1);
    let mut turn = 0;

    while !current.is_empty() {
        let mut new = FxHashMap::default();

        for (state, worlds) in current {
            let (p, s) = state[turn];
            for (step, w) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let p = (p + step) % 10;
                let s = s + p + 1;
                let worlds = worlds * w;

                if s >= 21 {
                    win[turn] += worlds;
                } else {
                    let mut state = state.clone();
                    state[turn] = (p, s);
                    *new.entry(state).or_default() += worlds;
                }
            }
        }

        current = new;
        turn ^= 1;
    }

    win.into_iter().max().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Player 1 starting position: 4
Player 2 starting position: 8"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 444356092776315);
    }
}
