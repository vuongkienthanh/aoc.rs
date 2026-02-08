use crate::is_triangle;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    input.into_iter().filter(|x| is_triangle(*x)).count()
}
