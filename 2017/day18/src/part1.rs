use crate::parsing::{Item, Target, parse_input};
use fxhash::FxHashMap as Map;

pub fn process(_input: &str) -> isize {
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
    registers: Map<char, isize>,
    idx: usize,
    last_sound_played: isize,
}

impl Duet {
    fn new(items: Vec<Item>) -> Self {
        Duet {
            items,
            registers: Map::default(),
            idx: 0,
            last_sound_played: 0,
        }
    }
    fn get_value(&self, t: &Target) -> isize {
        match *t {
            Target::Register(x) => self.registers.get(&x).cloned().unwrap_or_default(),
            Target::Value(x) => x,
        }
    }
    fn run_once(&mut self) -> Option<isize> {
        match self.items[self.idx] {
            Item::Snd(ref t) => {
                let sound = self.get_value(t);
                self.last_sound_played = sound;
                self.idx += 1;
            }
            Item::Set(a, ref t) => {
                let b = self.get_value(t);
                *self.registers.entry(a).or_default() = b;
                self.idx += 1;
            }
            Item::Add(a, ref t) => {
                let b = self.get_value(t);
                *self.registers.entry(a).or_default() += b;
                self.idx += 1;
            }
            Item::Mul(a, ref t) => {
                let b = self.get_value(t);
                *self.registers.entry(a).or_default() *= b;
                self.idx += 1;
            }
            Item::Mod(a, ref t) => {
                let b = self.get_value(t);
                *self.registers.entry(a).or_default() %= b;
                self.idx += 1;
            }
            Item::Rcv(a) => {
                if self.registers.get(&a).cloned().unwrap_or_default() != 0 {
                    return Some(self.last_sound_played);
                }
                self.idx += 1;
            }
            Item::Jgz(ref a, ref b) => {
                let a = self.get_value(a);
                let b = self.get_value(b);
                if a > 0 {
                    self.idx = self.idx.strict_add_signed(b);
                } else {
                    self.idx += 1;
                }
            }
        }

        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 4);
    }
}
