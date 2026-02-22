use crate::parsing::parse_input;
use crate::run;

pub fn process(_input: &str) -> usize {
    let (algo, image) = parse_input(_input);
    run(algo, image, 2)
}

