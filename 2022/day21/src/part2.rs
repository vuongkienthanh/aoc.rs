use crate::parsing::{Op, Operand, parse_input};

pub fn process(_input: &str) -> usize {
    let (mut hm, mut current) = parse_input(_input);
    hm.remove("humn");
    let root = current
        .iter()
        .enumerate()
        .find_map(|(i, (monkey, _, _, _))| (*monkey == "root").then_some(i))
        .unwrap();
    current[root].1 = Op::Eq;
    loop {
        let mut new = vec![];
        for (monkey, op, a, b) in &current {
            match (a, b) {
                (Operand::Name(x), b) if hm.contains_key(x) => {
                    let i = hm.get(x).unwrap();
                    new.push((*monkey, op.clone(), Operand::Value(*i), b.clone()));
                    continue;
                }
                (a, Operand::Name(x)) if hm.contains_key(x) => {
                    let i = hm.get(x).unwrap();
                    new.push((*monkey, op.clone(), a.clone(), Operand::Value(*i)));
                    continue;
                }
                (Operand::Value(a), Operand::Value(b)) => match op {
                    Op::Add => {
                        hm.insert(monkey, a + b);
                    }
                    Op::Sub => {
                        hm.insert(monkey, a - b);
                    }
                    Op::Mul => {
                        hm.insert(monkey, a * b);
                    }
                    Op::Div => {
                        hm.insert(monkey, a / b);
                    }
                    _ => new.push((*monkey, op.clone(), Operand::Value(*a), Operand::Value(*b))),
                },
                (a, b) => new.push((*monkey, op.clone(), a.clone(), b.clone())),
            }
        }

        if new == current {
            break;
        }
        current = new;
    }
    let root = current
        .iter()
        .enumerate()
        .find_map(|(i, (monkey, _, _, _))| (*monkey == "root").then_some(i))
        .unwrap();
    match current.remove(root) {
        (_, _, Operand::Name(a), Operand::Value(b))
        | (_, _, Operand::Value(b), Operand::Name(a)) => hm.insert(a, b),
        _ => panic!(),
    };
    while !current.is_empty() {
        let mut new = vec![];
        for (monkey, op, a, b) in current {
            if let Some(i) = hm.get(monkey) {
                match (op, a, b) {
                    (Op::Add, Operand::Name(a), Operand::Value(b))
                    | (Op::Add, Operand::Value(b), Operand::Name(a)) => {
                        hm.insert(a, i - b);
                    }
                    (Op::Sub, Operand::Name(a), Operand::Value(b)) => {
                        hm.insert(a, b + i);
                    }
                    (Op::Sub, Operand::Value(b), Operand::Name(a)) => {
                        hm.insert(a, b - i);
                    }
                    (Op::Mul, Operand::Name(a), Operand::Value(b))
                    | (Op::Mul, Operand::Value(b), Operand::Name(a)) => {
                        hm.insert(a, i / b);
                    }
                    (Op::Div, Operand::Name(a), Operand::Value(b)) => {
                        hm.insert(a, b * i);
                    }
                    (Op::Div, Operand::Value(b), Operand::Name(a)) => {
                        hm.insert(a, b / i);
                    }
                    _ => panic!(),
                }
            } else {
                new.push((monkey, op, a, b));
            }
        }

        current = new;
    }
    *hm.get("humn").unwrap()
}
