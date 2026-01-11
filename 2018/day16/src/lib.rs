pub mod parsing;
pub mod part1;
pub mod part2;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

const OPCODE_LIST: [Opcode; 16] = [
    Opcode::addr,
    Opcode::addi,
    Opcode::mulr,
    Opcode::muli,
    Opcode::banr,
    Opcode::bani,
    Opcode::borr,
    Opcode::bori,
    Opcode::setr,
    Opcode::seti,
    Opcode::gtir,
    Opcode::gtri,
    Opcode::gtrr,
    Opcode::eqir,
    Opcode::eqri,
    Opcode::eqrr,
];

struct Computer {
    registers: [usize; 4],
}
impl Computer {
    fn new() -> Self {
        Computer { registers: [0; 4] }
    }
    fn from_arr(registers: [usize; 4]) -> Self {
        Computer { registers }
    }
    fn run(&mut self, opcode: &Opcode, a: usize, b: usize, c: usize) {
        match *opcode {
            Opcode::addr => self.registers[c] = self.registers[a] + self.registers[b],
            Opcode::addi => self.registers[c] = self.registers[a] + b,
            Opcode::mulr => self.registers[c] = self.registers[a] * self.registers[b],
            Opcode::muli => self.registers[c] = self.registers[a] * b,
            Opcode::banr => self.registers[c] = self.registers[a] & self.registers[b],
            Opcode::bani => self.registers[c] = self.registers[a] & b,
            Opcode::borr => self.registers[c] = self.registers[a] | self.registers[b],
            Opcode::bori => self.registers[c] = self.registers[a] | b,
            Opcode::setr => self.registers[c] = self.registers[a],
            Opcode::seti => self.registers[c] = a,
            Opcode::gtir => self.registers[c] = if a > self.registers[b] { 1 } else { 0 },
            Opcode::gtri => self.registers[c] = if self.registers[a] > b { 1 } else { 0 },
            Opcode::gtrr => {
                self.registers[c] = if self.registers[a] > self.registers[b] {
                    1
                } else {
                    0
                }
            }
            Opcode::eqir => self.registers[c] = if a == self.registers[b] { 1 } else { 0 },
            Opcode::eqri => self.registers[c] = if self.registers[a] == b { 1 } else { 0 },
            Opcode::eqrr => {
                self.registers[c] = if self.registers[a] == self.registers[b] {
                    1
                } else {
                    0
                }
            }
        }
    }
}
