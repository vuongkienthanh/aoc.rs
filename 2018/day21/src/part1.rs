use day19::Device;
use day19::parsing::{Opcode, parse_input};

pub fn process(_input: &str) -> usize {
    let (p, cmds) = parse_input(_input);
    let mut device = Device::new(p);

    // stop at first eqrr to halt
    loop {
        let pointer = device.get_pointer();
        if let Some((cmd, a, b, c)) = cmds.get(pointer) {
            if (*cmd, *a, *b, *c) == (Opcode::eqrr, 5, 0, 1) {
                return device.registers[5];
            }
            device.run(cmd, *a, *b, *c);
        } else {
            break;
        }
    }
    panic!("should have an answer")
}
