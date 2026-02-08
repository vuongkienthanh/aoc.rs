pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::{Item, Target};

pub struct Computer {
    pub registers: [isize; 4],
    pub i: usize,
    pub program: Vec<Item>,
    pub optimized_program: Vec<Item>,
}

impl Computer {
    pub fn new(program: Vec<Item>) -> Self {
        let mut c = Self {
            registers: [0; 4],
            i: 0,
            program,
            optimized_program: vec![],
        };
        c.optimize();
        c
    }
    pub fn optimize(&mut self) {
        self.optimized_program = self.program.clone();
        for (i, v) in self.program.windows(5).enumerate() {
            match (v[0], v[1], v[2], v[3], v[4]) {
                (
                    Item::inc(r1),
                    Item::dec(r2),
                    Item::jnz(Target::Register(r3), Target::Value(v3)),
                    Item::dec(r4),
                    Item::jnz(Target::Register(r5), Target::Value(v5)),
                ) if r2 == r3 && r4 == r5 && v3 == -2 && v5 == -5 => {
                    self.optimized_program[i] = Item::optimized_add_product(r1, r2, r4);
                    self.optimized_program[i + 1] = Item::none;
                    self.optimized_program[i + 2] = Item::none;
                    self.optimized_program[i + 3] = Item::none;
                    self.optimized_program[i + 4] = Item::none;
                }
                _ => (),
            }
        }
        let optimized_clone = self.optimized_program.clone();
        for (i, v) in optimized_clone.windows(3).enumerate() {
            match (v[0], v[1], v[2]) {
                (
                    Item::inc(r1),
                    Item::dec(r2),
                    Item::jnz(Target::Register(r3), Target::Value(v3)),
                )
                | (
                    Item::dec(r2),
                    Item::inc(r1),
                    Item::jnz(Target::Register(r3), Target::Value(v3)),
                ) if r2 == r3 && v3 == -2 => {
                    self.optimized_program[i] = Item::optimized_add(r1, r2);
                    self.optimized_program[i + 1] = Item::none;
                    self.optimized_program[i + 2] = Item::none;
                }
                _ => (),
            }
        }
    }

    pub fn run(&mut self) {
        self.optimized_program[self.i].run(self)
    }

    pub fn run_loop(&mut self) {
        while self.i < self.program.len() {
            self.run();
        }
    }
    pub fn run_loop_debug(&mut self) {
        while self.i < self.program.len() {
            self.run();
            println!("registers {:?}, i: {}", &self.registers, self.i);
        }
    }
}

impl Item {
    fn run(self, computer: &mut Computer) {
        match self {
            Item::cpy(t, r) => {
                match t {
                    Target::Value(v) => computer.registers[r] = v,
                    Target::Register(r2) => computer.registers[r] = computer.registers[r2],
                };
                computer.i += 1;
            }
            Item::inc(r) => {
                computer.registers[r] += 1;
                computer.i += 1;
            }
            Item::dec(r) => {
                computer.registers[r] -= 1;
                computer.i += 1;
            }
            Item::jnz(t1, t2) => {
                let a = match t1 {
                    Target::Value(x) => x,
                    Target::Register(r) => computer.registers[r],
                };
                if a != 0 {
                    match t2 {
                        Target::Value(x) => computer.i = computer.i.strict_add_signed(x),
                        Target::Register(r) => {
                            computer.i = computer.i.strict_add_signed(computer.registers[r])
                        }
                    };
                } else {
                    computer.i += 1;
                }
            }
            Item::tgl(t) => {
                let away = match t {
                    Target::Value(x) => x,
                    Target::Register(r) => computer.registers[r],
                };
                let away_index = computer.i.strict_add_signed(away);
                if away_index < computer.program.len() {
                    match computer.program[away_index] {
                        Item::inc(r) => computer.program[away_index] = Item::dec(r),
                        Item::dec(r) => computer.program[away_index] = Item::inc(r),
                        Item::tgl(Target::Register(r)) => {
                            computer.program[away_index] = Item::inc(r)
                        }
                        Item::jnz(ref t, Target::Register(r)) => {
                            computer.program[away_index] = Item::cpy(*t, r)
                        }
                        Item::cpy(ref t, r) => {
                            computer.program[away_index] = Item::jnz(*t, Target::Register(r))
                        }
                        _ => (),
                    }
                }
                computer.i += 1;
                computer.optimize();
            }
            Item::optimized_add_product(r1, r2, r3) => {
                computer.registers[r1] += computer.registers[r2] * computer.registers[r3];
                computer.registers[r2] = 0;
                computer.registers[r3] = 0;
                computer.i += 5;
            }
            Item::optimized_add(r1, r2) => {
                computer.registers[r1] += computer.registers[r2];
                computer.registers[r2] = 0;
                computer.i += 3;
            }
            Item::none => (),
        }
    }
}
