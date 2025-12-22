use crate::parsing::{Target, parse_input};
use crate::{Bot, factory};
use fxhash::{FxHashMap as Map, FxHashSet as Set};


pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut factory = factory(input);
    let mut ans = 1;
    let mut seen = Set::default();

    for bot_number in factory.keys().cloned().collect::<Vec<_>>() {
        resolve_bot(bot_number, &mut factory, &mut ans, &mut seen);
    }
    ans
}

fn resolve_bot(
    bot_number: usize,
    factory: &mut Map<usize, Bot>,
    ans: &mut usize,
    seen: &mut Set<usize>,
) {
    let b = factory.get(&bot_number).unwrap();
    if b.chips.len() == 2 {
        let mut b = b.clone();
        b.chips.sort_unstable();
        let low = b.chips[0];
        let high = b.chips[1];

        match b.low_target {
            Target::Bot(low_bot_number) => {
                let low_bot_chips = factory.get(&low_bot_number).unwrap().chips;
                if !low_bot_chips.contains(&low) {
                    factory.get_mut(&low_bot_number).unwrap().chips.push(low);
                    resolve_bot(low_bot_number, factory, ans, seen);
                }
            }
            Target::Output(x) if [0, 1, 2].contains(&x) => {
                if seen.insert(x) {
                    *ans *= low;
                }
            }
            _ => (),
        }
        match b.high_target {
            Target::Bot(high_bot_number) => {
                let high_bot_chips = factory.get(&high_bot_number).unwrap().chips;
                if !high_bot_chips.contains(&high) {
                    factory.get_mut(&high_bot_number).unwrap().chips.push(high);
                    resolve_bot(high_bot_number, factory, ans, seen);
                }
            }
            Target::Output(x) if [0, 1, 2].contains(&x) => {
                if seen.insert(x) {
                    *ans *= high;
                }
            }
            _ => (),
        }
    }
}
