pub mod parsing;
pub mod part1;
pub mod part2;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Opcode {
    add,
    mul,
    halt,
    unknown,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            1 => Opcode::add,
            2 => Opcode::mul,
            99 => Opcode::halt,
            _ => Opcode::unknown,
        }
    }
}

pub struct Computer {
    pub prog: Vec<usize>,
    pointer: usize,
}

impl Computer {
    pub fn new(prog: Vec<usize>) -> Self {
        Computer { prog, pointer: 0 }
    }
    pub fn new_with(mut prog: Vec<usize>, a: usize, b: usize) -> Self {
        prog[1] = a;
        prog[2] = b;
        Computer { prog, pointer: 0 }
    }
    pub fn run(&mut self) -> bool {
        match Opcode::from(self.prog[self.pointer]) {
            Opcode::add => {
                let a = self.prog[self.pointer + 1];
                let b = self.prog[self.pointer + 2];
                let c = self.prog[self.pointer + 3];
                self.prog[c] = self.prog[a] + self.prog[b];
                self.pointer += 4;
            }
            Opcode::mul => {
                let a = self.prog[self.pointer + 1];
                let b = self.prog[self.pointer + 2];
                let c = self.prog[self.pointer + 3];
                self.prog[c] = self.prog[a] * self.prog[b];
                self.pointer += 4;
            }
            Opcode::halt => return true,
            Opcode::unknown => panic!("unknown opcode"),
        }
        false
    }
    pub fn run_to_finish(&mut self) {
        loop {
            if self.run() {
                break;
            }
        }
    }
}
