use crate::parsing::parse_input;
use std::collections::BTreeSet;
use crate::{iterate_map, standby_map};

pub fn process(_input: &str) -> usize {
    let mut map = parse_input(_input);
    let rows = map.len();
    let cols = map[0].len();
    let start = (0, 1);
    let end = (rows - 1, cols - 2);
    let mut step = 0;

    let mut current = BTreeSet::from([start]);
    loop {
        if current.iter().any(|loc| *loc == end) {
            break;
        }
        iterate_map(&mut map);
        let mut new = BTreeSet::new();
        for loc in current {
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

    current = BTreeSet::from([end]);
    loop {
        if current.iter().any(|loc| *loc == start) {
            break;
        }
        iterate_map(&mut map);
        let mut new = BTreeSet::new();
        for loc in current {
            if map[loc.0][loc.1].is_empty() {
                new.insert(loc);
            }
            for adj in [
                (loc.0 - 1, loc.1),
                ((loc.0 + 1).min(rows - 1), loc.1),
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

    current = BTreeSet::from([start]);
    loop {
        if current.iter().any(|loc| *loc == end) {
            break;
        }
        iterate_map(&mut map);
        let mut new = BTreeSet::new();
        for loc in current {
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
    step
}