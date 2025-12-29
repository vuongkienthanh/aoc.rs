use crate::parsing::{Item, Target, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let len = input.len();
    let mut coprocessor = Coprocessor::new(input);
    let mut ans = 0;
    loop {
        if coprocessor.run_once() {
            ans += 1;
        }
        if coprocessor.idx >= len {
            break ans;
        }
    }
}

struct Coprocessor {
    idx: usize,
    items: Vec<Item>,
    registers: [isize; 8],
}
impl Coprocessor {
    fn new(items: Vec<Item>) -> Self {
        Coprocessor {
            idx: 0,
            items,
            registers: [0isize; 8],
        }
    }
    fn get_value(&self, t: &Target) -> isize {
        match *t {
            Target::Register(x) => self.registers[x],
            Target::Value(x) => x,
        }
    }
    fn run_once(&mut self) -> bool {
        match self.items[self.idx] {
            Item::Set(r, ref t) => {
                self.registers[r] = self.get_value(t);
                self.idx += 1;
                false
            }
            Item::Sub(r, ref t) => {
                self.registers[r] -= self.get_value(t);
                self.idx += 1;
                false
            }
            Item::Mul(r, ref t) => {
                self.registers[r] *= self.get_value(t);
                self.idx += 1;
                true
            }
            Item::Jnz(ref t1, ref t2) => {
                if self.get_value(t1) != 0 {
                    self.idx = self.idx.strict_add_signed(self.get_value(t2));
                } else {
                    self.idx += 1;
                }
                false
            }
        }
    }
}
