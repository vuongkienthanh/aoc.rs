pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::FxHashMap as Map;
use parsing::Item;

fn create_map<'a>(input: Vec<Item<'a>>) -> Map<&'a str, Vec<&'a str>> {
    let mut map = Map::default();
    for (lhs, rhs) in input {
        map.insert(lhs, rhs);
    }
    map
}
