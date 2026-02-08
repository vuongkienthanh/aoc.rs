use crate::parsing::parse_input;
use crate::{Gates, Wires, run_instructions};

pub fn process(_input: &str) -> usize {
    let instructions = parse_input(_input);
    let mut wires = Wires::new();
    let mut pending_gates = Gates::new();
    run_instructions(&instructions, &mut wires, &mut pending_gates);
    wires.remove(&"a").unwrap() as usize
}
