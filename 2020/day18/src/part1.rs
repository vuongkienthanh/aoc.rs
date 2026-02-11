use crate::parsing::{Item, Op, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input.into_iter().map(|i| value(i)).sum()
}

fn value(item: Item) -> usize {
    match item {
        Item::Unit(x) => x,
        Item::Group { nums, ops } => {
            let mut nums = nums.into_iter();
            let mut a = value(nums.next().unwrap());
            for (op, b) in ops.into_iter().zip(nums) {
                match op {
                    Op::Add => a += value(b),
                    Op::Mul => a *= value(b),
                }
            }
            a
        }
    }
}
