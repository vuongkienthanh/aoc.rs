use crate::parsing::parse_input;
use crate::run;

pub fn process(_input: &str) -> u16 {
    let instructions = parse_input(_input);
    let mut wires = run(instructions);
    wires.remove(&"a").unwrap()
}
