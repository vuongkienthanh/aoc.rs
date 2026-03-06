use crate::Op;
use crate::parsing::parse_input2;

/// repr item as divisible part;
#[derive(Debug)]
pub struct Item {
    pub div2: usize,
    pub div3: usize,
    pub div5: usize,
    pub div7: usize,
    pub div11: usize,
    pub div13: usize,
    pub div17: usize,
    pub div19: usize,
}

#[derive(Debug)]
pub struct Monkey2 {
    pub items: Vec<Item>,
    pub op: Op,
    pub div: usize,
    pub iftrue: usize,
    pub iffalse: usize,
}

impl Monkey2 {
    fn operate(&self, mut item: Item) -> Item {
        match self.op {
            Op::Add(x) => {
                item.div2 = (item.div2 + x) % 2;
                item.div3 = (item.div3 + x) % 3;
                item.div5 = (item.div5 + x) % 5;
                item.div7 = (item.div7 + x) % 7;
                item.div11 = (item.div11 + x) % 11;
                item.div13 = (item.div13 + x) % 13;
                item.div17 = (item.div17 + x) % 17;
                item.div19 = (item.div19 + x) % 19;
            }
            Op::Mul(x) => {
                item.div2 = (item.div2 * x) % 2;
                item.div3 = (item.div3 * x) % 3;
                item.div5 = (item.div5 * x) % 5;
                item.div7 = (item.div7 * x) % 7;
                item.div11 = (item.div11 * x) % 11;
                item.div13 = (item.div13 * x) % 13;
                item.div17 = (item.div17 * x) % 17;
                item.div19 = (item.div19 * x) % 19;
            }
            Op::Square => {
                item.div2 = (item.div2 * item.div2) % 2;
                item.div3 = (item.div3 * item.div3) % 3;
                item.div5 = (item.div5 * item.div5) % 5;
                item.div7 = (item.div7 * item.div7) % 7;
                item.div11 = (item.div11 * item.div11) % 11;
                item.div13 = (item.div13 * item.div13) % 13;
                item.div17 = (item.div17 * item.div17) % 17;
                item.div19 = (item.div19 * item.div19) % 19;
            }
        }
        item
    }
    fn test(&self, item: &Item) -> bool {
        match self.div {
            2 => item.div2 == 0,
            3 => item.div3 == 0,
            5 => item.div5 == 0,
            7 => item.div7 == 0,
            11 => item.div11 == 0,
            13 => item.div13 == 0,
            17 => item.div17 == 0,
            19 => item.div19 == 0,
            _ => panic!(),
        }
    }
}

pub fn process(_input: &str) -> usize {
    let mut input = parse_input2(_input);
    let mut ans = vec![0; input.len()];
    for _round in 0..10_000 {
        for i in 0..input.len() {
            let monkey = input.get_mut(i).unwrap();
            let items: Vec<_> = monkey.items.extract_if(.., |_| true).collect();
            let mut targets = vec![];
            for mut item in items {
                ans[i] += 1;
                item = monkey.operate(item);
                if monkey.test(&item) {
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
    ans.sort_unstable();
    let a = ans.pop().unwrap();
    let b = ans.pop().unwrap();
    a * b
}
