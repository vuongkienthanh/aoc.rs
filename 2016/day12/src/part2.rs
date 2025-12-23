use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut computer: Computer = Computer::new(input);
    computer.registers[2] = 1;
    computer.run_loop();
    computer.registers[0] as usize
}
