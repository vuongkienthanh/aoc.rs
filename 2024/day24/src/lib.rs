pub mod part1;
pub mod part2;
use std::collections::HashMap;
#[derive(Eq, PartialEq, Debug, Clone)]
enum Side {
    Lhs,
    Rhs,
}

#[derive(Debug, Clone)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    lhs: (&'a str, Option<usize>),
    rhs: (&'a str, Option<usize>),
    op: Op,
    out: (&'a str, Option<usize>),
}

type WireVal<'a> = HashMap<&'a str, usize>;
type WireInLoc<'a> = HashMap<&'a str, Vec<(&'a str, Side)>>;
type WireOutLoc<'a> = HashMap<&'a str, &'a str>;
type Gates<'a> = HashMap<&'a str, Gate<'a>>;

fn parse(_input: &str) -> (WireVal, WireInLoc, WireOutLoc, Gates) {
    let (input_wires, input_gates) = _input.split_once("\n\n").unwrap();
    let mut wire_in_loc = HashMap::<&str, Vec<(&str, Side)>>::new();
    let mut wire_out_loc = HashMap::<&str, &str>::new();
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
        wire_in_loc.entry(lhs).or_default().push((gate, Side::Lhs));
        wire_in_loc.entry(rhs).or_default().push((gate, Side::Rhs));
        wire_out_loc.insert(out, gate);
        gates.insert(
            gate,
            Gate {
                lhs: (lhs, None),
                rhs: (rhs, None),
                op,
                out: (out, None),
            },
        );
    }
    let wire_val = input_wires
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            let val = val.parse::<usize>().unwrap();
            (name, val)
        })
        .collect::<HashMap<_, _>>();
    (wire_val, wire_in_loc, wire_out_loc, gates)
}
