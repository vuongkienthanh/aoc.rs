use crate::{monkey_business, Op};
use crate::parsing::parse_input;
use crate::part1::Monkey;

#[derive(Debug)]
pub struct Monkey2 {
    pub items: Vec<Vec<usize>>,
    pub op: Op,
    pub iftrue: usize,
    pub iffalse: usize,
}

impl Monkey2 {
    fn from_monkey1(m: Monkey, len: usize) -> Monkey2 {
        Monkey2 {
            items: m.items.into_iter().map(|v| vec![v; len]).collect(),
            op: m.op,
            iftrue: m.iftrue,
            iffalse: m.iffalse,
        }
    }
    fn operate(&self, item: &mut [usize], div_map: &[usize]) {
        match self.op {
            Op::Add(x) => {
                item.iter_mut()
                    .zip(div_map)
                    .for_each(|(item, d)| *item = (*item + x) % d);
            }
            Op::Mul(x) => {
                item.iter_mut()
                    .zip(div_map)
                    .for_each(|(item, d)| *item = (*item * x) % d);
            }
            Op::Square => {
                item.iter_mut()
                    .zip(div_map)
                    .for_each(|(item, d)| *item = (*item * *item) % d);
            }
        }
    }
    fn test(&self, item: &[usize], i: usize) -> bool {
        item[i] == 0
    }
}
pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let div_map: Vec<_> = input.iter().map(|m| m.div).collect();
    let len = input.len();
    let mut input: Vec<_> = input
        .into_iter()
        .map(|m| Monkey2::from_monkey1(m, len))
        .collect();
    let mut ans = vec![0; len];
    for _round in 0..10_000 {
        for i in 0..len {
            let monkey = input.get_mut(i).unwrap();
            let mut targets = vec![];
            for mut item in std::mem::take(&mut monkey.items) {
                ans[i] += 1;
                monkey.operate(&mut item, &div_map);
                if monkey.test(&item, i) {
                    targets.push((monkey.iftrue, item));
                } else {
                    targets.push((monkey.iffalse, item));
                }
            }
            for (to, item) in targets {
                input.get_mut(to).unwrap().items.push(item);
            }
        }
    }
    monkey_business(ans)
}