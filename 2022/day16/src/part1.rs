use crate::parsing::{encode, parse_input};
use fxhash::FxHashMap;

pub fn process(_input: &str) -> usize {
    let (map, aa) = encode(parse_input(_input));
    let mut current = FxHashMap::from_iter([((0u64, aa), 0usize)]);
    for i in 0..30 {
        let mut new: FxHashMap<(u64, u64), usize> = FxHashMap::default();

        for ((opened, loc), pressure) in current {
            let (p, path) = map.get(&loc).unwrap();
            // open valve
            if *p != 0 && opened & (1 << loc) == 0 {
                let pressure = pressure + (29 - i) * p;
                new.entry((opened | (1 << loc), loc))
                    .and_modify(|x| *x = (*x).max(pressure))
                    .or_insert(pressure);
            }
            // goto next
            for next_loc in path {
                // every 4 steps should open one valve
                // performance hack
                if opened.count_ones() >= i as u32 / 4 {
                    new.entry((opened, *next_loc))
                        .and_modify(|x| *x = (*x).max(pressure))
                        .or_insert(pressure);
                }
            }
        }

        current = new;
    }
    current.into_values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 1651);
    }
}
