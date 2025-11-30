use crate::adj::adj4;
use crate::parsing::{G, Item, Point};
use std::collections::HashMap;

#[derive(Default, Clone, Copy)]
pub enum WalkResult {
    #[default]
    Unknown,
    DeadEnd,
    Dst {
        step: usize,
        to: Point,
    },
}

pub type Cache = HashMap<Point, [WalkResult; 4]>;

pub fn walk(from: Point, g: &G, cache: &mut Cache) -> [WalkResult; 4] {
    let mut data = cache.get(&from).cloned().unwrap_or_default();

    'a: for (i, (wr, adj)) in data.iter_mut().zip(adj4(from)).enumerate() {
        if *wr != WalkResult::Unknown {
            continue;
        }
        if g[adj] == Item::Wall {
            *wr = WalkResult::DeadEnd;
            cache.entry(from).or_default()[i] = WalkResult::DeadEnd;
            continue;
        }

        let mut prev = from;
        let mut curr = adj;
        let mut step = 0;
        while g[curr] == Item::Space {
            step += 1;
            if let Some(next) = adj4(curr)
                .into_iter()
                .find(|p| g[*p] != Item::Wall && *p != prev)
            {
                prev = curr;
                curr = next;
            } else {
                *wr = WalkResult::DeadEnd;
                cache.entry(from).or_default()[i] = WalkResult::DeadEnd;
                continue 'a;
            }
        }
        match g[curr] {
            Item::Number(_) | Item::Cross => {
                step += 1;
                *wr = WalkResult::Dst { step, to: curr };
                cache.entry(from).or_default()[i] = WalkResult::Dst { step, to: curr };
                cache.entry(curr).or_default()[door(prev, curr)] =
                    WalkResult::Dst { step, to: from };
            }
            _ => panic!("should not be space"),
        }
    }

    data
}

fn door(prev: Point, curr: Point) -> usize {
    if prev.0 == curr.0 {
        if prev.1 > curr.1 { 3 } else { 2 }
    } else if prev.0 > curr.1 {
        1
    } else {
        0
    }
}