use crate::adj::adj4;
use crate::parsing::{G, Item, Point};
use std::collections::HashMap;

#[derive(Default, Clone, Copy)]
pub enum WalkResult {
    #[default]
    DeadEnd,
    Dst {
        step: usize,
        to: Point,
    },
}

pub type Cache = HashMap<Point, [WalkResult; 4]>;

pub fn walk(from: Point, g: &G, cache: &mut Cache) -> [WalkResult; 4] {
    if let Some(data) = cache.get(&from) {
        data.clone()
    } else {
        let mut data = [WalkResult::default(); 4];

        'a: for (wr, adj) in data.iter_mut().zip(adj4(from)) {
            if g[adj] == Item::Wall {
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
                    continue 'a;
                }
            }
            match g[curr] {
                Item::Number(_) | Item::Cross => {
                    step += 1;
                    *wr = WalkResult::Dst { step, to: curr };
                }
                _ => panic!("should not be space"),
            }
        }
        cache.insert(from, data.clone());

        data
    }
}
