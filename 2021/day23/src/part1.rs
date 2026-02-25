use crate::{State, available_paths, ignore_rooms, score};
use fxhash::FxHashMap;

pub fn process(_input: &str) -> usize {
    let mut current = vec![State::from(_input)];
    let mut seen: FxHashMap<[u8; 8], usize> = FxHashMap::default();
    seen.insert(current.first().unwrap().locations.clone(), 0);
    let mut ans = usize::MAX;
    while !current.is_empty() {
        let mut new = vec![];

        for state in current {
            if state.is_done() {
                ans = ans.min(state.score);
                continue;
            }
            for i in 0..8 {
                if i == state.last_moved {
                    continue;
                }
                let from = state.locations[i];
                let ignored = ignore_rooms(i);
                let iscore = score(i);
                for (to, step, blockers) in available_paths(from) {
                    if ignored.contains(&to) {
                        continue;
                    }
                    if blockers.into_iter().any(|b| state.locations.contains(&b)) {
                        continue;
                    }
                }
            }
        }

        current = new;
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
