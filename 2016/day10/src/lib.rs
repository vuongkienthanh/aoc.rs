pub mod parsing;
pub mod part1;
pub mod part2;
use crate::parsing::{Input, Target};
use std::collections::HashMap;
use tinyvec::ArrayVec;

#[derive(Debug, Default, Clone)]
struct Bot {
    chips: ArrayVec<[usize; 2]>,
    low_target: Target,
    high_target: Target,
}

fn factory(input: Vec<Input>) -> HashMap<usize, Bot> {
    let mut ans = HashMap::new();
    for i in input {
        match i {
            Input::Init(value, bot_number) => {
                let bot: &mut Bot = ans.entry(bot_number).or_default();
                bot.chips.push(value);
            }
            Input::Bot(bot_number, low_target, high_target) => {
                let bot: &mut Bot = ans.entry(bot_number).or_default();
                bot.low_target = low_target;
                bot.high_target = high_target;
            }
        }
    }
    ans
}
