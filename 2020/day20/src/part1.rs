use crate::build_image;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let image = build_image(input);
    let f = image.first().unwrap();
    let b = image.last().unwrap();
    f.first().unwrap().0 * f.last().unwrap().0 * b.first().unwrap().0 * b.last().unwrap().0
}
