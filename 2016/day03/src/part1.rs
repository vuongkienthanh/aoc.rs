use crate::is_triangle;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());

    input
        .into_iter()
        .filter(|x| is_triangle(*x))
        .count()
}

