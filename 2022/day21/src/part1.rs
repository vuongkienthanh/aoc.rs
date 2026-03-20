use crate::parsing::{Op, Operand, parse_input};

pub fn process(_input: &str) -> usize {
    let (mut hm, mut current) = parse_input(_input);
    while !current.is_empty() {
        let mut new = vec![];
        for (monkey, op, a, b) in current {
            match (a, b) {
                (Operand::Name(x), b) if hm.contains_key(x) => {
                    let i = hm.get(x).unwrap();
                    new.push((monkey, op, Operand::Value(*i), b));
                    continue;
                }
                (a, Operand::Name(x)) if hm.contains_key(x) => {
                    let i = hm.get(x).unwrap();
                    new.push((monkey, op, a, Operand::Value(*i)));
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
                    _ => panic!(),
                },
                (a, b) => new.push((monkey, op, a, b)),
            }
        }

        current = new;
    }

    *hm.get("root").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 152);
    }
}
