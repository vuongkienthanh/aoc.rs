use crate::parsing::{Target, parse_input};
use crate::{Bot, factory};
use fxhash::FxHashMap as Map;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut factory = factory(input);
    let target = (17, 61);
    for bot_number in factory.keys().cloned().collect::<Vec<_>>() {
        if let Some(n) = resolve_bot(bot_number, &mut factory, target) {
            return n;
        }
    }
    panic!("should have an answer")
}

fn resolve_bot(
    bot_number: usize,
    factory: &mut Map<usize, Bot>,
    target: (usize, usize),
) -> Option<usize> {
    let b = factory.get(&bot_number).unwrap();
    if b.chips.len() == 2 {
        let mut b = b.clone();
        b.chips.sort_unstable();
        let low = b.chips[0];
        let high = b.chips[1];
        if (low, high) == target {
            return Some(bot_number);
        }
        if let Target::Bot(low_bot_number) = b.low_target {
            let low_bot_chips = factory.get(&low_bot_number).unwrap().chips;
            if !low_bot_chips.contains(&low) {
                factory.get_mut(&low_bot_number).unwrap().chips.push(low);
                if let Some(n) = resolve_bot(low_bot_number, factory, target) {
                    return Some(n);
                }
            }
        }
        if let Target::Bot(high_bot_number) = b.high_target {
            let high_bot_chips = factory.get(&high_bot_number).unwrap().chips;
            if !high_bot_chips.contains(&high) {
                factory.get_mut(&high_bot_number).unwrap().chips.push(high);
                if let Some(n) = resolve_bot(high_bot_number, factory, target) {
                    return Some(n);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2"#
    }
    #[rstest]
    fn test_process(fixture: &str) {
        let (_, input) = parse_input(fixture).unwrap();
        let mut factory = factory(input);
        let target = (2, 5);
        for bot_number in factory.keys().cloned().collect::<Vec<_>>() {
            if let Some(n) = resolve_bot(bot_number, &mut factory, target) {
                assert_eq!(n, 2)
            }
        }
    }
}
