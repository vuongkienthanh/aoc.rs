use crate::parsing::parse_input;
use crate::{Computer, OPCODE_LIST, Opcode};
use fxhash::FxHashSet;

pub fn process(_input: &str) -> usize {
    let (blocks, test) = parse_input(_input);

    let mut possibilities: Vec<Vec<Opcode>> = (0..16).map(|_| OPCODE_LIST.to_vec()).collect();

    for (before, [opcode, a, b, c], after) in blocks {
        possibilities.get_mut(opcode).unwrap().retain(|op| {
            let mut comp = Computer::from_arr(before);
            comp.run(op, a, b, c);
            comp.registers == after
        })
    }

    let mut seen = FxHashSet::default();
    while seen.len() != 16 {
        for possible in possibilities.iter_mut() {
            if possible.len() == 1 {
                seen.insert(possible[0]);
            } else {
                possible.retain(|x| !seen.contains(x));
            }
        }
    }

    let opcodes: Vec<_> = possibilities.into_iter().map(|x| x[0]).collect();

    let mut comp = Computer::new();
    for [i, a, b, c] in test {
        comp.run(opcodes.get(i).unwrap(), a, b, c);
    }
    comp.registers[0]
}
