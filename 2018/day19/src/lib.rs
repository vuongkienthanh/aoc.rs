pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::Opcode;

pub struct Device {
    p: usize,
    pub registers: [usize; 6],
}

impl Device {
    pub fn new(p: usize) -> Self {
        Device {
            p,
            registers: [0; 6],
        }
    }
    pub fn new_with_a(p: usize, i: usize) -> Self {
        let mut device = Device::new(p);
        device.registers[0] = i;
        device
    }
    pub fn get_pointer(&self) -> usize {
        self.registers[self.p]
    }
    pub fn run(&mut self, opcode: &Opcode, a: usize, b: usize, c: usize) {
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
        self.registers[self.p] += 1;
    }
    fn run_cmds(&mut self, cmds: &[(Opcode, usize, usize, usize)]) -> bool {
        if let Some((cmd, a, b, c)) = cmds.get(self.get_pointer()) {
            // println!("run cmd {cmd:?} {a} {b} {c}");
            self.run(cmd, *a, *b, *c);
            false
        } else {
            true
        }
    }
}
