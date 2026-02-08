pub mod parsing;
pub mod part1;
pub mod part2;
use crate::parsing::{Operand, Operation};
use std::collections::{HashMap, VecDeque};

type Wires<'a> = HashMap<&'a str, u16>;
type Gates<'a> = VecDeque<PendingGate<'a>>;

struct PendingGate<'a> {
    target: &'a str,
    missing: Vec<&'a str>,
    ops: GateOp,
}

enum GateOp {
    Assign,
    AndMissingBoth,
    OrMissingBoth,
    And(u16),
    Or(u16),
    Lshift(u16),
    Rshift(u16),
    Not,
}

fn process_wires_and_pending_gates<'a>(
    k: &'a str,
    wires: &mut Wires<'a>,
    pending_gates: &mut Gates<'a>,
) {
    use GateOp::*;
    let val = wires.get(k).copied().unwrap();
    let mut new_ks = Vec::new();

    for _ in 0..pending_gates.len() {
        let mut gate = pending_gates.pop_front().unwrap();
        if gate.missing.contains(&k) {
            match gate.ops {
                AndMissingBoth => {
                    gate.missing.retain(|x| x != &k);
                    gate.ops = GateOp::And(val);
                    pending_gates.push_back(gate);
                }
                OrMissingBoth => {
                    gate.missing.retain(|x| x != &k);
                    gate.ops = GateOp::Or(val);
                    pending_gates.push_back(gate);
                }
                Assign => {
                    new_ks.push(gate.target);
                    wires.insert(gate.target, val);
                }
                And(i) => {
                    new_ks.push(gate.target);
                    wires.insert(gate.target, i & val);
                }
                Or(i) => {
                    new_ks.push(gate.target);
                    wires.insert(gate.target, i | val);
                }
                Lshift(i) => {
                    new_ks.push(gate.target);
                    wires.insert(gate.target, val << i);
                }
                Rshift(i) => {
                    new_ks.push(gate.target);
                    wires.insert(gate.target, val >> i);
                }
                Not => {
                    new_ks.push(gate.target);
                    wires.insert(gate.target, !val);
                }
            }
        } else {
            pending_gates.push_back(gate);
        }
    }

    for new_k in new_ks {
        process_wires_and_pending_gates(new_k, wires, pending_gates);
    }
}
fn run_instructions<'a>(
    instructions: &Vec<Operation<'a>>,
    wires: &mut Wires<'a>,
    pending_gates: &mut Gates<'a>,
) {
    use Operand::*;
    use Operation::*;
    for ins in instructions {
        match ins {
            Assign(Value(i), target) => {
                wires.insert(target, *i);
                process_wires_and_pending_gates(target, wires, pending_gates);
            }

            Assign(Name(k), target) => {
                if let Some(i) = wires.get(k).copied() {
                    wires.insert(target, i);
                    process_wires_and_pending_gates(target, wires, pending_gates);
                } else {
                    pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k],
                        ops: GateOp::Assign,
                    })
                }
            }
            And(Value(x), Operand::Value(y), target) => {
                wires.insert(target, x & y);
                process_wires_and_pending_gates(target, wires, pending_gates);
            }
            And(Name(k), Operand::Value(x), target) | And(Value(x), Operand::Name(k), target) => {
                if let Some(y) = wires.get(k).copied() {
                    wires.insert(target, x & y);
                    process_wires_and_pending_gates(target, wires, pending_gates);
                } else {
                    pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k],
                        ops: GateOp::And(*x),
                    })
                }
            }
            And(Name(k1), Name(k2), target) => {
                match (wires.get(k1).copied(), wires.get(k2).copied()) {
                    (Some(x), Some(y)) => {
                        wires.insert(target, x & y);
                        process_wires_and_pending_gates(target, wires, pending_gates);
                    }
                    (Some(x), None) => pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k2],
                        ops: GateOp::And(x),
                    }),
                    (None, Some(x)) => pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k1],
                        ops: GateOp::And(x),
                    }),
                    (None, None) => {
                        pending_gates.push_back(PendingGate {
                            target,
                            missing: vec![k1, k2],
                            ops: GateOp::AndMissingBoth,
                        });
                    }
                }
            }
            Or(Value(x), Value(y), target) => {
                wires.insert(target, x | y);
                process_wires_and_pending_gates(target, wires, pending_gates);
            }
            Or(Name(k), Value(x), target) | Or(Value(x), Name(k), target) => {
                if let Some(y) = wires.get(k).copied() {
                    wires.insert(target, x | y);
                    process_wires_and_pending_gates(target, wires, pending_gates);
                } else {
                    pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k],
                        ops: GateOp::Or(*x),
                    })
                }
            }
            Or(Name(k1), Name(k2), target) => {
                match (wires.get(k1).copied(), wires.get(k2).copied()) {
                    (Some(x), Some(y)) => {
                        wires.insert(target, x | y);
                        process_wires_and_pending_gates(target, wires, pending_gates);
                    }
                    (Some(x), None) => pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k2],
                        ops: GateOp::Or(x),
                    }),
                    (None, Some(x)) => pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k1],
                        ops: GateOp::Or(x),
                    }),
                    (None, None) => {
                        pending_gates.push_back(PendingGate {
                            target,
                            missing: vec![k1, k2],
                            ops: GateOp::OrMissingBoth,
                        });
                    }
                }
            }
            Lshift(k, x, target) => {
                if let Some(y) = wires.get(k).copied() {
                    wires.insert(target, y << x);
                    process_wires_and_pending_gates(target, wires, pending_gates);
                } else {
                    pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k],
                        ops: GateOp::Lshift(*x),
                    })
                }
            }
            Rshift(k, x, target) => {
                if let Some(y) = wires.get(k).copied() {
                    wires.insert(target, y >> x);
                    process_wires_and_pending_gates(target, wires, pending_gates);
                } else {
                    pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k],
                        ops: GateOp::Rshift(*x),
                    })
                }
            }
            Not(k, target) => {
                if let Some(from_val) = wires.get(k).copied() {
                    wires.insert(target, !from_val);
                    process_wires_and_pending_gates(target, wires, pending_gates);
                } else {
                    pending_gates.push_back(PendingGate {
                        target,
                        missing: vec![k],
                        ops: GateOp::Not,
                    })
                }
            }
        }
    }
}
