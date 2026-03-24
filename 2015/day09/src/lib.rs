pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::Map;

fn total_distance<'a>(routes: Vec<&'a str>, map: &Map<'a>) -> usize {
    routes
        .windows(2)
        .map(|v| {
            let (loc1, loc2) = (v[0], v[1]);
            map.get(loc1).unwrap().get(loc2).unwrap()
        })
        .sum()
}
