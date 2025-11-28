use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    println!("{input:?}");
    println!("{_rest:?}");
    assert!(_rest.is_empty());

    let mut computer = Computer::new(input);
    computer.registers[0] = 7;
    computer.run_loop_debug();

    computer.registers[0] as usize
}
