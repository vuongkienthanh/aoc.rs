use crate::build_map;
use crate::parsing::parse_input;
use crate::part1::need_ore;

pub fn process(_input: &str) -> usize {
    let map = build_map(parse_input(_input));
    let ore_have = 1_000_000_000_000;

    let mut low = ore_have / need_ore(1, &map);
    let mut high = 10 * low;
    while need_ore(high, &map) < ore_have {
        low = high;
        high = 10 * low;
    }
    while low < high - 1 {
        let mid = (low + high) / 2;
        let ore = need_ore(mid, &map);
        if ore < ore_have {
            low = mid;
        } else if ore > ore_have {
            high = mid;
        } else {
            break;
        }
    }
    low
}
