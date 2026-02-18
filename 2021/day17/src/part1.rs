use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_, _, max_y, _) = parse_input(_input);
    let max_y = max_y.unsigned_abs();
    max_y * (max_y - 1) / 2
}

