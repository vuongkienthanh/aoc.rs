pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::Mapping;
fn total_distance<'a>(routes: Vec<&'a str>, mapping: &'a Mapping<'a>) -> usize {
    routes
        .windows(2)
        .map(|v| {
            let (loc1, loc2) = (v[0], v[1]);
            mapping.get(loc1).unwrap().get(loc2).unwrap()
        })
        .sum()
}
