use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();

    let mut comp = Computer::new(input);
    comp.reg[0] = 1;
    loop {
        if comp.run() {
            return comp.reg[1];
        }
    }
}
