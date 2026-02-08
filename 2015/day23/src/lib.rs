pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::CMD;

struct Computer {
    reg: [usize; 2],
    i: usize,
    program: Vec<CMD>,
}

impl Computer {
    fn new(program: Vec<CMD>) -> Self {
        Self {
            reg: [0, 0],
            i: 0,
            program,
        }
    }
    fn run_cmd(&mut self, cmd: CMD) {
        println!("{}:{:?} [a,b]={:?}", self.i, cmd, self.reg);
        match cmd {
            CMD::hlf(r) => {
                self.reg[r] /= 2;
                self.i += 1;
            }
            CMD::tpl(r) => {
                self.reg[r] *= 3;
                self.i += 1;
            }
            CMD::inc(r) => {
                self.reg[r] += 1;
                self.i += 1;
            }
            CMD::jmpf(o) => self.i += o,
            CMD::jmpb(o) => self.i -= o,
            CMD::jief(r, o) => {
                if self.reg[r].is_multiple_of(2) {
                    self.i += o
                } else {
                    self.i += 1
                }
            }
            CMD::jieb(r, o) => {
                if self.reg[r].is_multiple_of(2) {
                    self.i -= o
                } else {
                    self.i += 1
                }
            }
            CMD::jiof(r, o) => {
                if self.reg[r] == 1 {
                    self.i += o
                } else {
                    self.i += 1
                }
            }
            CMD::jiob(r, o) => {
                if self.reg[r] == 1 {
                    self.i -= o
                } else {
                    self.i += 1
                }
            }
        }
    }
    fn run(&mut self) -> bool {
        if self.i >= self.program.len() {
            return true;
        }
        self.run_cmd(self.program[self.i]);
        false
    }
}
