use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut comp = Computer::new(input, 5);
    comp.run_to_finish();
    comp.output.into_iter().find(|x| *x > 0).unwrap() as usize
}
