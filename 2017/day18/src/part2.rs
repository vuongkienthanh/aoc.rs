use crate::parsing::{Item, Target, parse_input};
use fxhash::FxHashMap as Map;
use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut duet = Duet::new(input);

    loop {
        if let Some(x) = duet.run_once() {
            break x;
        }
    }
}
struct Duet {
    items: Vec<Item>,
    idx0: usize,
    registers0: Map<char, isize>,
    queue0: VecDeque<isize>,
    idx1: usize,
    registers1: Map<char, isize>,
    queue1: VecDeque<isize>,
    sent1: usize,
}

impl Duet {
    fn new(items: Vec<Item>) -> Self {
        let mut registers0 = Map::default();
        let mut registers1 = Map::default();
        registers0.insert('p', 0);
        registers1.insert('p', 1);
        Duet {
            items,
            idx0: 0,
            registers0,
            queue0: VecDeque::new(),
            idx1: 0,
            registers1,
            queue1: VecDeque::new(),
            sent1: 0,
        }
    }
    fn get_value(&self, program: usize, t: &Target) -> isize {
        match program {
            0 => match *t {
                Target::Register(x) => self.registers0.get(&x).cloned().unwrap_or_default(),
                Target::Value(x) => x,
            },
            1 => match *t {
                Target::Register(x) => self.registers1.get(&x).cloned().unwrap_or_default(),
                Target::Value(x) => x,
            },
            _ => panic!(),
        }
    }
    /// return if program is waiting
    fn run_0(&mut self) -> bool {
        match self.items[self.idx0] {
            Item::Snd(ref t) => {
                let val = self.get_value(0, t);
                self.queue1.push_back(val);
                self.idx0 += 1;
            }
            Item::Set(a, ref t) => {
                let b = self.get_value(0, t);
                *self.registers0.entry(a).or_default() = b;
                self.idx0 += 1;
            }
            Item::Add(a, ref t) => {
                let b = self.get_value(0, t);
                *self.registers0.entry(a).or_default() += b;
                self.idx0 += 1;
            }
            Item::Mul(a, ref t) => {
                let b = self.get_value(0, t);
                *self.registers0.entry(a).or_default() *= b;
                self.idx0 += 1;
            }
            Item::Mod(a, ref t) => {
                let b = self.get_value(0, t);
                *self.registers0.entry(a).or_default() %= b;
                self.idx0 += 1;
            }
            Item::Rcv(a) => {
                if let Some(x) = self.queue0.pop_front() {
                    *self.registers0.entry(a).or_default() = x;
                    self.idx0 += 1;
                } else {
                    return true;
                }
            }
            Item::Jgz(ref a, ref b) => {
                let a = self.get_value(0, a);
                let b = self.get_value(0, b);
                if a > 0 {
                    self.idx0 = self.idx0.strict_add_signed(b);
                } else {
                    self.idx0 += 1;
                }
            }
        }
        false
    }
    /// return if program is waiting
    fn run_1(&mut self) -> bool {
        match self.items[self.idx1] {
            Item::Snd(ref t) => {
                let val = self.get_value(1, t);
                self.queue0.push_back(val);
                self.idx1 += 1;
                self.sent1 += 1;
            }
            Item::Set(a, ref t) => {
                let b = self.get_value(1, t);
                *self.registers1.entry(a).or_default() = b;
                self.idx1 += 1;
            }
            Item::Add(a, ref t) => {
                let b = self.get_value(1, t);
                *self.registers1.entry(a).or_default() += b;
                self.idx1 += 1;
            }
            Item::Mul(a, ref t) => {
                let b = self.get_value(1, t);
                *self.registers1.entry(a).or_default() *= b;
                self.idx1 += 1;
            }
            Item::Mod(a, ref t) => {
                let b = self.get_value(1, t);
                *self.registers1.entry(a).or_default() %= b;
                self.idx1 += 1;
            }
            Item::Rcv(a) => {
                if let Some(x) = self.queue1.pop_front() {
                    *self.registers1.entry(a).or_default() = x;
                    self.idx1 += 1;
                } else {
                    return true;
                }
            }
            Item::Jgz(ref a, ref b) => {
                let a = self.get_value(1, a);
                let b = self.get_value(1, b);
                if a > 0 {
                    self.idx1 = self.idx1.strict_add_signed(b);
                } else {
                    self.idx1 += 1;
                }
            }
        }
        false
    }

    fn run_once(&mut self) -> Option<usize> {
        (self.run_0() && self.run_1()).then_some(self.sent1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 3);
    }
}
