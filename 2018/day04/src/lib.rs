pub mod parsing;
pub mod part1;
pub mod part2;
use fxhash::FxHashMap as Map;
use parsing::{Item, Time};

fn build_map(mut input: Vec<(Time, Item)>) -> Map<usize, Vec<(usize, usize)>> {
    input.sort_unstable_by_key(|x| x.0);
    let mut guard_map: Map<usize, Vec<(usize, usize)>> = Map::default();
    let mut shift = 0;
    let mut sleep_start = None;
    for ((_, _, h, m), item) in input {
        match item {
            Item::Shift(x) => {
                guard_map.entry(x).or_default();
                shift = x;
                if sleep_start.is_some() {
                    panic!("new shift should have no sleep_start");
                }
            }
            Item::Sleep => {
                if h != 0 {
                    panic!("sleep should be midnight")
                }
                sleep_start = Some(m);
            }
            Item::Wake => {
                if h != 0 {
                    panic!("wake should be midnight")
                }
                if let Some(s) = sleep_start {
                    guard_map.get_mut(&shift).unwrap().push((s, m));
                    sleep_start = None;
                } else {
                    panic!("should sleep before wake")
                }
            }
        }
    }
    guard_map
}
