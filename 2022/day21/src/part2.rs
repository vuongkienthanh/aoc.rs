use crate::parsing::{Op, Operand, parse_input};

pub fn process(_input: &str) -> usize {
    let (mut hm, mut current) = parse_input(_input);
    hm.remove("humn");
    let root = current
        .iter()
        .enumerate()
        .find_map(|(i, (monkey, _, _, _))| (*monkey == "root").then_some(i))
        .unwrap();
    let root = current.remove(root);
    loop {
        let mut changed = false;
        let mut new = vec![];
        for (monkey, op, a, b) in current {
            match (a, b) {
                (Operand::Name(x), b) if hm.contains_key(x) => {
                    let i = hm.get(x).unwrap();
                    new.push((monkey, op, Operand::Value(*i), b));
                    changed = true;
                    continue;
                }
                (a, Operand::Name(x)) if hm.contains_key(x) => {
                    let i = hm.get(x).unwrap();
                    new.push((monkey, op, a, Operand::Value(*i)));
                    changed = true;
                    continue;
                }
                (Operand::Value(a), Operand::Value(b)) => match op {
                    Op::Add => {
                        hm.insert(monkey, a + b);
                        changed = true;
                    }
                    Op::Sub => {
                        hm.insert(monkey, a - b);
                        changed = true;
                    }
                    Op::Mul => {
                        hm.insert(monkey, a * b);
                        changed = true;
                    }
                    Op::Div => {
                        hm.insert(monkey, a / b);
                        changed = true;
                    }
                },
                (a, b) => new.push((monkey, op, a, b)),
            }
        }
        current = new;
        if !changed {
            break;
        }
    }
    match root {
        (_, _, Operand::Name(a), Operand::Name(b)) => {
            if let Some(i) = hm.get(a) {
                hm.insert(b, *i);
            } else if let Some(i) = hm.get(b) {
                hm.insert(a, *i);
            } else {
                panic!()
            }
        }
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
