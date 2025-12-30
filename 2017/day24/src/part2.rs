use crate::parsing::parse_input;
use crate::{build_map, build_strength};
use fxhash::FxHashSet as Set;
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let map = build_map(&input);
    let str = build_strength(&input);

    let mut max_strength = usize::MIN;
    let mut max_length = usize::MIN;

    let mut current: Set<(usize, usize, BTreeSet<usize>)> = Set::default();
    current.insert((0, 0, BTreeSet::default()));

    while !current.is_empty() {
        let mut new = Set::default();

        for (need, strength, bridge) in current {
            match bridge.len().cmp(&max_length) {
                Ordering::Greater => {
                    max_length = bridge.len();
                    max_strength = strength;
                }
                Ordering::Equal => {
                    max_strength = max_strength.max(strength);
                }
                Ordering::Less => (),
            }

            if let Some(candidates) = map.get(&need) {
                for (candidate, next_need) in candidates {
                    if !bridge.contains(candidate) {
                        let mut new_bridge = bridge.clone();
                        new_bridge.insert(*candidate);
                        let new_strength = strength + str[*candidate];
                        new.insert((*next_need, new_strength, new_bridge));
                    }
                }
            }
        }

        current = new;
    }

    max_strength
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 19);
    }
}
