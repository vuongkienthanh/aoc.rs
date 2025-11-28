use crate::parsing::{parse_input};
use crate::Computer;

pub fn process(_input: &str) -> usize {
    let (_, input) = parse_input(_input).unwrap();
    let mut computer = Computer::new(input);
    computer.registers[0] = 12;
    computer.optimize();

    computer.run_loop_debug();

    computer.registers[0] as usize

}
