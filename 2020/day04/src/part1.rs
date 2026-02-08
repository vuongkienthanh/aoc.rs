use crate::parsing::parse_input;
use crate::{ORDER1, ORDER2};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .map(|group| group.into_iter().map(|(k, _)| k).collect::<Vec<_>>())
        .filter(|group| *group == ORDER1 || *group == ORDER2)
        .count()
}
