use crate::parse;

pub fn process(_input: &str) -> u16 {
    let input = parse(_input);
    input.into_iter().max().unwrap()
}
