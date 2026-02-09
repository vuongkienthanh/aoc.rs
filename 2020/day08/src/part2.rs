use crate::parsing::{Item, parse_input};
use fxhash::FxHashSet;

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    let mut ans = 0;
    for (i, item) in input.iter().enumerate() {
        let new_input = match item {
            Item::nop(x) => {
                let mut input = input.clone();
                input[i] = Item::jmp(*x);
                Some(input)
            }
            Item::jmp(x) => {
                let mut input = input.clone();
                input[i] = Item::nop(*x);
                Some(input)
            }
            _ => None,
        };
        if let Some(input) = new_input {
            let mut seen = FxHashSet::default();
            let mut i = 0;
            let mut acc = 0isize;
            let get_halted = loop {
                if !seen.insert(i) {
                    break false;
                }
                if i >= input.len() {
                    break true;
                }
                match input[i] {
                    Item::nop(_) => i += 1,
                    Item::acc(x) => {
                        acc += x;
                        i += 1;
                    }
                    Item::jmp(x) => i = i.checked_add_signed(x).unwrap(),
                }
            };
            if get_halted {
                ans = acc;
                break;
            }
        }
    }
    ans
}
