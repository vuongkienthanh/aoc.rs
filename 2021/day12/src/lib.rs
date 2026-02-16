pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::FxHashMap;
type Map<'a> = FxHashMap<&'a str, Vec<&'a str>>;

fn build_map<'a>(input: Vec<(&'a str, &'a str)>) -> Map<'a> {
    let mut map = Map::default();
    for (a, b) in input {
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }

    map
}
