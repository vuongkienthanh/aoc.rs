use std::collections::{BTreeMap, VecDeque};

pub fn parse(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

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
    rel,
    halt,
    unknown,
}

impl From<i64> for Opcode {
    fn from(value: i64) -> Self {
        match value {
            1 => Opcode::add,
            2 => Opcode::mul,
            3 => Opcode::input,
            4 => Opcode::output,
            5 => Opcode::jit,
            6 => Opcode::jif,
            7 => Opcode::lt,
            8 => Opcode::eq,
            9 => Opcode::rel,
            99 => Opcode::halt,
            _ => Opcode::unknown,
        }
    }
}

pub enum RunResult {
    Running,
    Output(i64),
    Halt,
}

pub struct Computer {
    pub prog: BTreeMap<usize, i64>,
    pointer: usize,
    relative_base: usize,
    input: VecDeque<i64>,
}

impl Computer {
    pub fn new(prog: Vec<i64>) -> Self {
        Computer {
            prog: prog.into_iter().enumerate().collect(),
            input: VecDeque::new(),
            pointer: 0,
            relative_base: 0,
        }
    }

    pub fn append_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn get_value(&self, modes: &mut i64, parameter_pointer: usize) -> i64 {
        let mode = *modes % 10;
        (*modes) /= 10;
        match mode {
            0 => {
                let loc = self.prog[&parameter_pointer] as usize;
                self.prog.get(&loc).cloned().unwrap_or(0)
            }
            1 => self.prog.get(&parameter_pointer).cloned().unwrap_or(0),
            2 => {
                let loc = self
                    .relative_base
                    .checked_add_signed(self.prog[&parameter_pointer] as isize)
                    .unwrap();
                self.prog.get(&loc).cloned().unwrap_or(0)
            }
            _ => panic!("unknown mode"),
        }
    }

    pub fn get_loc(&self, modes: &mut i64, parameter_pointer: usize) -> usize {
        let mode = *modes % 10;
        (*modes) /= 10;
        match mode {
            0 => self
                .prog
                .get(&parameter_pointer)
                .map(|x| *x as usize)
                .unwrap_or(0),
            1 => panic!("loc parameter should not be intermediate"),
            2 => self
                .relative_base
                .checked_add_signed(self.prog[&parameter_pointer] as isize)
                .unwrap(),

            _ => panic!("unknown mode"),
        }
    }

    pub fn run(&mut self) -> RunResult {
        let instruction = self.prog[&self.pointer];
        let opcode = instruction % 100;
        let mut modes = instruction / 100;
        match Opcode::from(opcode) {
            Opcode::add => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                let c = self.get_loc(&mut modes, self.pointer + 3);
                self.prog.insert(c, a + b);
                self.pointer += 4;
            }
            Opcode::mul => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                let c = self.get_loc(&mut modes, self.pointer + 3);
                self.prog.insert(c, a * b);
                self.pointer += 4;
            }
            Opcode::input => {
                let a = self.get_loc(&mut modes, self.pointer + 1);
                self.prog.insert(a, self.input.pop_front().unwrap());
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
                let c = self.get_loc(&mut modes, self.pointer + 3);
                if a < b {
                    self.prog.insert(c, 1);
                } else {
                    self.prog.insert(c, 0);
                }
                self.pointer += 4;
            }
            Opcode::eq => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                let b = self.get_value(&mut modes, self.pointer + 2);
                let c = self.get_loc(&mut modes, self.pointer + 3);
                if a == b {
                    self.prog.insert(c, 1);
                } else {
                    self.prog.insert(c, 0);
                }
                self.pointer += 4;
            }
            Opcode::rel => {
                let a = self.get_value(&mut modes, self.pointer + 1);
                self.relative_base = self.relative_base.checked_add_signed(a as isize).unwrap();
                self.pointer += 2;
            }
            Opcode::halt => return RunResult::Halt,
            Opcode::unknown => panic!("unknown opcode"),
        }
        RunResult::Running
    }
    pub fn long_run(&mut self) -> Option<i64> {
        loop {
            match self.run() {
                RunResult::Running => (),
                RunResult::Output(a) => break Some(a),
                RunResult::Halt => break None,
            }
        }
    }
}
