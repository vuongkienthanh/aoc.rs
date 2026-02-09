use crate::parsing::{Item, parse_input};
use fxhash::FxHashSet;

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    let mut seen = FxHashSet::default();
    let mut i = 0;
    let mut acc = 0isize;

    loop {
        if seen.insert(i) && i < input.len() {
            match input[i] {
                Item::nop(_) => i += 1,
                Item::acc(x) => {
                    acc += x;
                    i += 1;
                }
                Item::jmp(x) => i = i.checked_add_signed(x).unwrap(),
            }
        } else {
            break;
        }
    }
    acc
}
