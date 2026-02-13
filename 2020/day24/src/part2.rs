use crate::parsing::{Item, parse_input};
use fxhash::{FxHashMap, FxHashSet};

type Room = FxHashMap<(isize, isize), bool>;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut room: FxHashSet<(isize, isize)> = input
        .into_iter()
        .fold(Room::default(), |mut acc, line| {
            let (mut ns, mut ew) = (0, 0);
            for c in line {
                match c {
                    Item::E => ew += 1,
                    Item::W => ew -= 1,
                    Item::NW => ns += 1,
                    Item::SE => ns -= 1,
                    Item::NE => {
                        ns += 1;
                        ew += 1;
                    }
                    Item::SW => {
                        ns -= 1;
                        ew -= 1;
                    }
                }
            }
            acc.entry((ns, ew))
                .and_modify(|x| *x = !(*x))
                .or_insert(true);
            acc
        })
        .into_iter()
        .filter_map(|(k, x)| x.then_some(k))
        .collect();

    for _ in 0..100 {
        let mut new_room = FxHashSet::default();
        let mut whites: FxHashMap<(isize, isize), usize> = FxHashMap::default();

        for (ns, ew) in &room {
            let adjs = [
                (*ns, *ew - 1),
                (*ns, *ew + 1),
                (*ns + 1, *ew),
                (*ns + 1, *ew + 1),
                (*ns - 1, *ew),
                (*ns - 1, *ew - 1),
            ];
            let mut black_count = 0;
            for adj in adjs {
                if room.contains(&adj) {
                    black_count += 1;
                } else {
                    *whites.entry(adj).or_default() += 1;
                }
            }
            if !(black_count == 0 || black_count > 2) {
                new_room.insert((*ns, *ew));
            }
        }
        for (white, black_count) in whites {
            if black_count == 2 {
                new_room.insert(white);
            }
        }

        room = new_room;
    }

    room.len()
}
