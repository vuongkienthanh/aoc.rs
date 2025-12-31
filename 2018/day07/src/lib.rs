pub mod parsing;
pub mod part1;
pub mod part2;
use fxhash::FxHashMap;

#[derive(Default, Debug)]
pub struct Node {
    parent: Vec<char>,
    children: Vec<char>,
}

pub type Map = FxHashMap<char, Node>;
fn build_map(input: Vec<(char, char)>) -> Map {
    let mut map = Map::default();
    for (a, b) in input {
        map.entry(a).or_default().children.push(b);
        map.entry(b).or_default().parent.push(a);
    }
    map
}

fn find_roots(map: &Map) -> Vec<char> {
    map.iter()
        .filter_map(|(k, v)| v.parent.is_empty().then_some(k))
        .cloned()
        .collect::<Vec<_>>()
}
