use crate::parsing::{Op, Operand, OperandList, parse_input};
use crate::run;

pub fn process(_input: &str) -> u16 {
    let mut instructions = parse_input(_input);
    let mut wires = run(instructions.clone());
    let a = wires.remove(&"a").unwrap();
    for (op, v, target) in &mut instructions {
        if matches!(op, Op::Assign) && *target == "b" {
            *v = OperandList::One(Operand::Value(a));
            break;
        }
    }
    let mut wires = run(instructions);
    wires.remove(&"a").unwrap()
}
