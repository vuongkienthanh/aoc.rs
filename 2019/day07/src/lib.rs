pub mod parsing;
pub mod part1;
pub mod part2;
use std::collections::VecDeque;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Opcode {
    add,
    mul,
    input,
    output,
    jit,
    jif,
    lt,
    eq,
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
            5 => Opcode::jit,
            6 => Opcode::jif,
            7 => Opcode::lt,
            8 => Opcode::eq,
            99 => Opcode::halt,
            _ => Opcode::unknown,
        }
    }
}

pub enum RunResult {
    Running,
    Output(isize),
    Halt,
}

pub struct Computer {
    pub prog: Vec<isize>,
    pointer: usize,
    input: VecDeque<isize>,
}

impl Computer {
    pub fn new(prog: Vec<isize>) -> Self {
        Computer {
            prog,
            input: VecDeque::new(),
            pointer: 0,
        }
    }

    pub fn append_input(&mut self, input: isize) {
        self.input.push_back(input);
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
    pub fn run(&mut self) -> RunResult {
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
                self.prog[a as usize] = self.input.pop_front().unwrap();
                self.pointer += 2;
            }
            Opcode::output => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                self.pointer += 2;
                return RunResult::Output(a);
            }
            Opcode::jit => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                if a != 0 {
                    self.pointer = b as usize;
                } else {
                    self.pointer += 3;
                }
            }
            Opcode::jif => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                if a == 0 {
                    self.pointer = b as usize;
                } else {
                    self.pointer += 3;
                }
            }
            Opcode::lt => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                let c = self.prog[self.pointer + 3];
                if a < b {
                    self.prog[c as usize] = 1;
                } else {
                    self.prog[c as usize] = 0;
                }
                self.pointer += 4;
            }
            Opcode::eq => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                let c = self.prog[self.pointer + 3];
                if a == b {
                    self.prog[c as usize] = 1;
                } else {
                    self.prog[c as usize] = 0;
                }
                self.pointer += 4;
            }
            Opcode::halt => return RunResult::Halt,
            Opcode::unknown => panic!("unknown opcode"),
        }
        RunResult::Running
    }
    pub fn long_run(&mut self) -> Option<isize> {
        loop {
            match self.run() {
                RunResult::Running => (),
                RunResult::Output(a) => break Some(a),
                RunResult::Halt => break None,
            }
        }
    }
}
