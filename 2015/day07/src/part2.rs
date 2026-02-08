use crate::parsing::{Operation, parse_input};
use crate::{Gates, Wires, process_wires_and_pending_gates, run_instructions};

pub fn process(_input: &str) -> usize {
    let mut instructions = parse_input(_input);
    let mut wires = Wires::new();
    let mut pending_gates = Gates::new();
    run_instructions(&instructions, &mut wires, &mut pending_gates);

    let a_val = wires.remove(&"a").unwrap();
    wires.clear();
    pending_gates.clear();

    wires.insert("b", a_val);
    process_wires_and_pending_gates("b", &mut wires, &mut pending_gates);

    instructions.retain(|i| !matches!(i, Operation::Assign(_, "b")));

    run_instructions(&instructions, &mut wires, &mut pending_gates);
    wires.remove(&"a").unwrap() as usize
}
