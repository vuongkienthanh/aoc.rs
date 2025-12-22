pub mod parsing;
pub mod part1;
pub mod part2;
use crate::parsing::{Input, Target};
use tinyvec::ArrayVec;
use fxhash::FxHashMap as Map;

#[derive(Debug, Default, Clone)]
struct Bot {
    chips: ArrayVec<[usize; 2]>,
    low_target: Target,
    high_target: Target,
}

fn factory(input: Vec<Input>) -> Map<usize, Bot> {
    let mut ans = Map::default();
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
