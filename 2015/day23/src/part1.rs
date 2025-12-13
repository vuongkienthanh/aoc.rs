use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());


    let mut comp = Computer::new(input);
    loop {
        if comp.run() {
            return comp.reg[1];
        }
    }
}
