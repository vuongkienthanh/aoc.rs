use crate::parsing::{Item, parse_input};
use aoc_helper::assembly::Computer;

pub fn process(_input: &str) -> usize {
    let (_, input) = parse_input(_input).unwrap();
    let mut computer: Computer<4, Item> = Computer::new(input);
    computer.registers[2] =1;
    computer.run(false);
    computer.registers[0]

}
