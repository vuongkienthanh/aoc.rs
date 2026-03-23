use crate::parsing::{Point, parse_input};
use std::collections::{BTreeMap, BTreeSet};

pub fn process(_input: &str) -> usize {
    let mut current = parse_input(_input);
    let mut check = [
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
    ];
    for round in 0..10 {
        let mut map: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
        let mut new = BTreeSet::new();
        for (r, c) in &current {
            if [
                (*r - 1, *c - 1),
                (*r - 1, *c),
                (*r - 1, *c + 1),
                (*r, *c - 1),
                (*r, *c + 1),
                (*r + 1, *c - 1),
                (*r + 1, *c),
                (*r + 1, *c + 1),
            ]
            .into_iter()
            .all(|(r, c)| !current.contains(&(r, c)))
            {
                new.insert((*r, *c));
            } else {
                if let Some(arr) = check.iter().find(|arr| {
                    arr.iter()
                        .map(|(dr, dc)| (r + dr, c + dc))
                        .all(|(r, c)| !current.contains(&(r, c)))
                }) {
                    let (dr, dc) = arr[1];
                    map.entry((r + dr, c + dc)).or_default().push((*r, *c));
                } else {
                    new.insert((*r, *c));
                }
            }
        }
        for (to, v_from) in map {
            if v_from.len() == 1 {
                new.insert(to);
            } else {
                new.extend(v_from);
            }
        }
        current = new;
        check.rotate_left(1);
    }

    let (mut min_r, mut max_r, mut min_c, mut max_c) =
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
    for (r, c) in &current {
        min_r = min_r.min(*r);
        max_r = max_r.max(*r);
        min_c = min_c.min(*c);
        max_c = max_c.max(*c);
    }
    ((max_r - min_r + 1) * (max_c - min_c + 1)) as usize - current.len()
}