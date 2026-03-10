use crate::parsing::{encode, parse_input};
use fxhash::FxHashMap;

pub fn process(_input: &str) -> usize {
    let (map, aa) = encode(parse_input(_input));
    let num = map.values().filter(|(p, _)| *p > 0).count() as u32;
    // every span should open one valve
    let span = (26 / num).max(2);
    println!("num = {num} => {span}");
    let mut current = FxHashMap::from_iter([((0u64, aa, aa), 0usize)]);
    for i in 0..26 {
        let mut move_loc_a: FxHashMap<(u64, u64, u64), usize> = FxHashMap::default();
        let mut move_loc_b: FxHashMap<(u64, u64, u64), usize> = FxHashMap::default();

        for ((opened, loc_a, loc_b), pressure) in current {
            let (p, path) = map.get(&loc_a).unwrap();
            // open valve
            if *p != 0 && opened & (1 << loc_a) == 0 {
                let pressure = pressure + (25 - i) * p;
                move_loc_a
                    .entry((opened | (1 << loc_a), loc_a, loc_b))
                    .and_modify(|x| *x = (*x).max(pressure))
                    .or_insert(pressure);
            }
            // goto next
            for next_loc in path {
                // every span should open one valve
                if opened.count_ones() >= (i as u32 / span) {
                    move_loc_a
                        .entry((opened, *next_loc, loc_b))
                        .and_modify(|x| *x = (*x).max(pressure))
                        .or_insert(pressure);
                }
            }
        }
        for ((opened, loc_a, loc_b), pressure) in move_loc_a {
            let (p, path) = map.get(&loc_b).unwrap();
            // open valve
            // [you, elephant] is the same as [elephant, you]
            if *p != 0 && opened & (1 << loc_b) == 0 {
                let pressure = pressure + (25 - i) * p;
                move_loc_b
                    .entry((opened | (1 << loc_b), loc_a.min(loc_b), loc_a.max(loc_b)))
                    .and_modify(|x| *x = (*x).max(pressure))
                    .or_insert(pressure);
            }
            // goto next
            for next_loc in path {
                // [you, elephant] is the same as [elephant, you]
                move_loc_b
                    .entry((opened, loc_a.min(*next_loc), loc_a.max(*next_loc)))
                    .and_modify(|x| *x = (*x).max(pressure))
                    .or_insert(pressure);
            }     
        }

        current = move_loc_b;
        println!("{i} {}", current.len());
    }
    current.into_values().max().unwrap()
}