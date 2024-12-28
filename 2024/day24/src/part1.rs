use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Side {
    Lhs,
    Rhs,
}

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate<'a> {
    lhs: Option<usize>,
    rhs: Option<usize>,
    op: Op,
    out: &'a str,
}

pub fn process(_input: &str) -> usize {
    let (input_wires, input_gates) = _input.split_once("\n\n").unwrap();

    let mut wires = HashMap::<&str, Vec<(&str, Side)>>::new();
    let mut gates = HashMap::new();
    let mut zs = vec![];

    for gate in input_gates.lines() {
        let mut v = gate.split_ascii_whitespace();
        let lhs = v.next().unwrap();
        let op = match v.next().unwrap() {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!(),
        };
        let rhs = v.next().unwrap();
        v.next().unwrap();
        let out = v.next().unwrap();
        wires.entry(lhs).or_default().push((gate, Side::Lhs));
        wires.entry(rhs).or_default().push((gate, Side::Rhs));
        gates.insert(
            gate,
            Gate {
                lhs: None,
                rhs: None,
                op,
                out,
            },
        );
    }

    let mut input_wires = input_wires
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            let value = value.parse::<usize>().unwrap();
            (name, value)
        })
        .collect::<VecDeque<_>>();

    while let Some((wire, value)) = input_wires.pop_front() {
        if wire.starts_with("z") {
            zs.push((wire, value));
        }
        if let Some(dst) = wires.get(&wire) {
            for (gate, side) in dst {
                match side {
                    Side::Lhs => gates.get_mut(gate).unwrap().lhs.replace(value),
                    Side::Rhs => gates.get_mut(gate).unwrap().rhs.replace(value),
                };
                let gate = gates.get(gate).unwrap();
                if let (Some(l), Some(r)) = (gate.lhs, gate.rhs) {
                    let new_value = match gate.op {
                        Op::And => l & r,
                        Op::Or => l | r,
                        Op::Xor => l ^ r,
                    };
                    input_wires.push_back((gate.out, new_value));
                }
            }
        }
    }
    zs.sort_unstable_by(|a, b| b.0.cmp(a.0));
    zs.into_iter()
        .map(|x| x.1)
        .reduce(|acc, ele| acc << 1 | ele)
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;
        assert_eq!(process(input), 4);
    }
    #[test]
    fn test_process_2() {
        let input = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;
        assert_eq!(process(input), 2024);
    }
}
