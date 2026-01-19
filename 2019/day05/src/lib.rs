pub mod parsing;
pub mod part1;
pub mod part2;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Opcode {
    add,
    mul,
    input,
    output,
    halt,
    unknown,
}

impl From<isize> for Opcode {
    fn from(value: isize) -> Self {
        match value {
            1 => Opcode::add,
            2 => Opcode::mul,
            3 => Opcode::input,
            4 => Opcode::output,
            99 => Opcode::halt,
            _ => Opcode::unknown,
        }
    }
}

pub struct Computer {
    pub prog: Vec<isize>,
    pointer: usize,
    input: isize,
    pub output: Vec<isize>,
}

impl Computer {
    pub fn new(prog: Vec<isize>, input: isize) -> Self {
        Computer {
            prog,
            input,
            output: vec![],
            pointer: 0,
        }
    }
    pub fn get_value(&self, modes: &mut isize, parameter_pointer: usize) -> isize {
        let mode = *modes % 10;
        (*modes) /= 10;
        match mode {
            0 => {
                let loc = self.prog[parameter_pointer] as usize;
                self.prog[loc]
            }
            1 => self.prog[parameter_pointer],
            _ => panic!("unknown mode"),
        }
    }
    pub fn run(&mut self) -> bool {
        let instruction = self.prog[self.pointer];
        let opcode = instruction % 100;
        let mut modes = instruction / 100;
        match Opcode::from(opcode) {
            Opcode::add => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                let c = self.prog[self.pointer + 3];
                self.prog[c as usize] = a + b;
                self.pointer += 4;
            }
            Opcode::mul => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                let c = self.prog[self.pointer + 3];
                self.prog[c as usize] = a * b;
                self.pointer += 4;
            }
            Opcode::input => {
                let a = self.prog[self.pointer + 1];
                self.prog[a as usize] = self.input;
                self.pointer += 2;
            }
            Opcode::output => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                self.output.push(a);
                self.pointer += 2;
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
