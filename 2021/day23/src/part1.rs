use crate::State;
use fxhash::FxHashMap;

pub fn process(_input: &str) -> usize {
    let mut current = vec![State::from(_input)];
    let mut seen: FxHashMap<[u8; 8], usize> = FxHashMap::default();
    seen.insert(current.first().unwrap().normalized_locations(), 0);
    let mut ans = usize::MAX;
    while !current.is_empty() {
        let mut new = vec![];

        for state in current {
            if state.is_done() {
                ans = ans.min(state.score);
                continue;
            }
            for next_state in state.next_states() {
                let normalized = next_state.normalized_locations();
                if let Some(s) = seen.get(&normalized) {
                    if next_state.score < *s {
                        seen.insert(normalized, next_state.score);
                        new.push(next_state);
                    }
                } else {
                    seen.insert(normalized, next_state.score);
                    new.push(next_state);
                }
            }
        }

        current = new;
        // println!("{current:?}");
        // break;
    }
    ans
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
