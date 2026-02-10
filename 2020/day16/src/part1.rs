use crate::parsing::parse_input;
use crate::valid_ranges;

pub fn process(_input: &str) -> usize {
    let (class, _, nearby) = parse_input(_input);
    let valid_ranges = valid_ranges(&class);
    nearby
        .into_iter()
        .flatten()
        .filter_map(|x| {
            valid_ranges
                .iter()
                .all(|(a, b)| x < *a || x > *b)
                .then_some(x)
        })
        .sum()
}
