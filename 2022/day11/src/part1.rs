use crate::parsing::parse_input;
use crate::{monkey_business, Op};

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub op: Op,
    pub div: usize,
    pub iftrue: usize,
    pub iffalse: usize,
}

impl Monkey {
    fn operate(&self, item: usize) -> usize {
        match self.op {
            Op::Add(x) => item + x,
            Op::Mul(x) => item * x,
            Op::Square => item * item,
        }
    }
    fn test(&self, item: usize) -> bool {
        item.is_multiple_of(self.div)
    }
}

pub fn process(_input: &str) -> usize {
    let mut input = parse_input(_input);
    let mut ans = vec![0; input.len()];
    for _round in 0..20 {
        for i in 0..input.len() {
            let monkey = input.get_mut(i).unwrap();
            let mut targets = vec![];
            for mut item in std::mem::take(&mut monkey.items) {
                ans[i] += 1;
                item = monkey.operate(item) / 3;
                if monkey.test(item) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 10605);
    }
}
