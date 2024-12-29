pub mod part1;
pub mod part2;
use std::collections::HashMap;
#[derive(Debug, Clone)]
enum Side {
    Lhs,
    Rhs,
}

#[derive(Debug,Clone)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    lhs: Option<usize>,
    rhs: Option<usize>,
    op: Op,
    out: &'a str,
}

type WireVal<'a> = HashMap<&'a str, usize>;
type WireLoc<'a> = HashMap<&'a str, Vec<(&'a str, Side)>>;
type Gates<'a> = HashMap<&'a str, Gate<'a>>;

fn parse(_input: &str) -> (WireVal, WireLoc, Gates) {
    let (input_wires, input_gates) = _input.split_once("\n\n").unwrap();
    let wire_val = input_wires
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            let val = val.parse::<usize>().unwrap();
            (name, val)
        })
        .collect::<HashMap<_, _>>();
    let mut wire_loc = HashMap::<&str, Vec<(&str, Side)>>::new();
    let mut gates = HashMap::new();

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
        wire_loc.entry(lhs).or_default().push((gate, Side::Lhs));
        wire_loc.entry(rhs).or_default().push((gate, Side::Rhs));
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
    (wire_val, wire_loc, gates)
}
