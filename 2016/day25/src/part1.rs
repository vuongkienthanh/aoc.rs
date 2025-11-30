use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    // println!("{input:?}");
    // println!("{_rest:?}");
    assert!(_rest.is_empty());

    todo!("part1")
}

