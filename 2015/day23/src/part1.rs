use crate::parsing::{CMD, parse_input};

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());


    println!("{input:?}");
    todo!()

    // let mut comp = Computer::new(input);
    // loop {
    //     comp.run_i()
    // }
    //
}

struct Computer {
    reg: [usize; 2],
    i: isize,
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
        match cmd {
            CMD::hlf(r) => {
                self.reg[r] = self.reg[r] / 2;
                self.i += 1;
            }
            CMD::tpl(r) => {
                self.reg[r] = self.reg[r] * 3;
                self.i += 1;
            }
            CMD::inc(r) => {
                self.reg[r] = self.reg[r] + 1;
                self.i += 1;
            }
            CMD::jmp(o) => {
                self.i = self.i + o;
            }
            CMD::jie(r, o) => {
                if self.reg[r] % 2 == 0 {
                    self.i = self.i + o;
                }
            }
            CMD::jio(r, o) => {
                if self.reg[r] == 1 {
                    self.i = self.i + o;
                }
            }
        }
    }
    fn run_i(&mut self) {
        self.run_cmd(self.program[self.i as usize])
    }
}
