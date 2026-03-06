pub mod parsing;
pub mod part1;
pub mod part2;
use fxhash::FxHashMap;
use parsing::{Item, Op, Operand, OperandList};

fn run<'a>(mut input: Vec<Item<'a>>) -> FxHashMap<&'a str, u16> {
    use Op::*;
    use Operand::*;
    use OperandList::*;
    let mut wires = FxHashMap::default();

    while !input.is_empty() {
        let mut new = vec![];

        for item in input {
            match item {
                (Assign, One(Value(v)), target) => {
                    wires.insert(target, v);
                }
                (And, Two(Value(v1), Value(v2)), target) => {
                    wires.insert(target, v1 & v2);
                }
                (Or, Two(Value(v1), Value(v2)), target) => {
                    wires.insert(target, v1 | v2);
                }
                (Lshift, Two(Value(v1), Value(v2)), target) => {
                    wires.insert(target, v1 << v2);
                }
                (Rshift, Two(Value(v1), Value(v2)), target) => {
                    wires.insert(target, v1 >> v2);
                }
                (Not, One(Value(v)), target) => {
                    wires.insert(target, !v);
                }
                (op, One(Name(n)), target) => {
                    if let Some(v) = wires.get(&n) {
                        new.push((op, One(Value(*v)), target));
                    } else {
                        new.push((op, One(Name(n)), target));
                    }
                }
                (op, Two(Name(n), x2), target) => {
                    if let Some(v) = wires.get(&n) {
                        new.push((op, Two(Value(*v), x2), target));
                    } else {
                        new.push((op, Two(Name(n), x2), target));
                    }
                }
                (op, Two(x1, Name(n)), target) => {
                    if let Some(v) = wires.get(&n) {
                        new.push((op, Two(x1, Value(*v)), target));
                    } else {
                        new.push((op, Two(x1, Name(n)), target));
                    }
                }
                _ => panic!("should not exists"),
            }
        }
        input = new;
    }
    wires
}
