use crate::parsing::{Item, Op, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input.into_iter().map(|i| value(i)).sum()
}

fn value(item: Item) -> usize {
    match item {
        Item::Unit(x) => x,
        Item::Group { nums, ops } => {
            let mut new_nums = vec![];
            let mut nums = nums.into_iter().peekable();
            let mut a = value(nums.next().unwrap());
            for op in ops {
                match op {
                    Op::Add => {
                        let b = value(nums.next().unwrap());
                        a += b;
                    }
                    Op::Mul => {
                        new_nums.push(a);
                        a = value(nums.next().unwrap());
                    }
                }
            }
            a * new_nums.into_iter().product::<usize>()
        }
    }
}
