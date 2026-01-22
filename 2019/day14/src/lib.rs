pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::Item;
use std::collections::BTreeMap;

fn build_map<'a>(
    input: Vec<(Vec<Item<'a>>, Item<'a>)>,
) -> BTreeMap<&'a str, (usize, BTreeMap<&'a str, usize>)> {
    let mut map = BTreeMap::default();
    for (left, (c, right)) in input {
        let mut from_map = BTreeMap::new();
        for (count, f) in left {
            from_map.insert(f, count);
        }
        map.insert(right, (c, from_map));
    }
    map
}
