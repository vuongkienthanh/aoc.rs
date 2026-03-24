use crate::parsing::parse_input;
use std::collections::BTreeSet;
use crate::{iterate_map, standby_map};

pub fn process(_input: &str) -> usize {
    let mut map = parse_input(_input);
    let loc = (0, 1);
    let end = (map.len() - 1, map[0].len() - 2);

    let mut current = BTreeSet::from([loc]);
    let mut step = 0;

    'ans: loop {
        iterate_map(&mut map);
        let mut new = BTreeSet::new();
        for loc in current {
            if loc == end {
                break 'ans step;
            }
            if map[loc.0][loc.1].is_empty() {
                new.insert(loc);
            }
            for adj in [
                (loc.0.saturating_sub(1), loc.1),
                (loc.0 + 1, loc.1),
                (loc.0, loc.1 - 1),
                (loc.0, loc.1 + 1),
            ] {
                if map[adj.0][adj.1].is_empty() {
                    new.insert(adj);
                }
            }
        }

        current = new;
        step += 1;
        standby_map(&mut map);
    }
}