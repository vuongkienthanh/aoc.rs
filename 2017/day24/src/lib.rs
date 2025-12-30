pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::FxHashMap as Map;

fn build_map(input: &[(usize, usize)]) -> Map<usize, Vec<(usize, usize)>> {
    let mut map: Map<usize, Vec<(usize, usize)>> = Map::default();

    for (i, (a, b)) in input.into_iter().enumerate() {
        map.entry(*a).or_default().push((i, *b));
        map.entry(*b).or_default().push((i, *a));
    }

    map
}
fn build_strength(input: &[(usize, usize)]) -> Vec<usize> {
    input.into_iter().map(|(a, b)| a + b).collect()
}
