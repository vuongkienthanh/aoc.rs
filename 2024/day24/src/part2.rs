use crate::{parse, Gate, Op, Side, WireVal};
use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    hash::Hash,
};

struct State<'a> {
    i: usize,
    changes: BTreeSet<Pair<'a>>,
    to_change: Option<Pair<'a>>,
    carry_gates: HashMap<&'a str, Gate<'a>>,
    carry_wires: HashMap<&'a str, usize>,
}

#[derive(Debug, Clone, Ord, PartialOrd)]
struct Pair<'a>(&'a str, &'a str);
impl Eq for Pair<'_> {}
impl PartialEq for Pair<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}
impl Hash for Pair<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

pub fn process(_input: &str) -> String {
    let (wire_val, wire_loc, mut gates) = parse(_input);
    let ((x, y, z), (xb, yb, zb)) = find_z(&wire_val);

    println!("x = {x} {xb:?}");
    println!("y = {y} {yb:?}");
    println!("z = {z} {zb:?}");

    for (k, v) in wire_val {
        for (gate, side) in wire_loc.get(&k).unwrap() {
            match side {
                Side::Lhs => gates.get_mut(gate).unwrap().lhs.replace(v),
                Side::Rhs => gates.get_mut(gate).unwrap().rhs.replace(v),
            };
        }
    }

    let mut visited: BTreeSet<BTreeSet<Pair>> = BTreeSet::new();
    let mut z45: BTreeSet<Pair> = BTreeSet::new();

    let mut deq = VecDeque::from([State {
        i: 0,
        changes: BTreeSet::new(),
        to_change: None,
        carry_gates: HashMap::new(),
        carry_wires: HashMap::new(),
    }]);

    'outer: while let Some(State {
        i,
        changes,
        to_change,
        carry_gates,
        carry_wires,
    }) = deq.pop_front()
    {
        if i == 1 {
            break;
        }

        println!("========================================");
        println!("state i={i} changes={changes:?}, to_change={to_change:?}, carry_gates={carry_gates:?}, carry_wires={carry_wires:?}");

        let mut new_changes = changes.clone();
        if let Some(p) = to_change.clone() {
            new_changes.insert(p);
        }
        if carry_wires.keys().any(|x| x.starts_with("z")) {
            println!("SKIP: invalid carry_wires that starts with z");
            continue;
        }
        if to_change.is_some() {
            // check if already seen this changes
            if visited.contains(&new_changes) {
                println!("SKIP: this (changes, to_change) is already seen");
                continue;
            } else {
                visited.insert(new_changes.clone());
            }
        }
        let xk = format!("x{i:0>2}");
        let yk = format!("y{i:0>2}");
        let zk = format!("z{i:0>2}");

        // build gates store
        // from i
        // and from prev carry_wires
        let mut this_gates = [xk.as_str(), yk.as_str()]
            .into_iter()
            .chain(carry_wires.keys().cloned())
            .map(|x| wire_loc.get(x).unwrap())
            .flat_map(|v| v.iter().map(|x| x.0))
            .collect::<BTreeSet<&str>>()
            .iter()
            .map(|x| (*x, gates.get(x).unwrap().clone()))
            .collect::<HashMap<&str, Gate>>();
        // and from prev carry_gates
        for (k, v) in &carry_gates {
            this_gates.insert(*k, v.clone());
        }

        // new candidates
        if changes.len() < 4 {
            for out in this_gates
                .values()
                .map(|x| x.out)
                .collect::<BTreeSet<&str>>()
                .iter()
                .combinations(2)
            {
                let value = if out[0] < out[1] {
                    Pair(out[0], out[1])
                } else {
                    Pair(out[1], out[0])
                };
                deq.push_back(State {
                    i,
                    changes: changes.clone(),
                    to_change: Some(value),
                    carry_gates: carry_gates.clone(),
                    carry_wires: carry_wires.clone(),
                });
            }
        }

        // swapping out wires if any
        if let Some(Pair(a, b)) = to_change {
            for gate in this_gates.values_mut() {
                if gate.out == a {
                    gate.out = b;
                } else if gate.out == b {
                    gate.out = a;
                }
            }
        }
        // fill gates
        for (k, v) in [(xk.as_str(), xb[i]), (yk.as_str(), yb[i])]
            .into_iter()
            .chain(carry_wires.iter().map(|(k, v)| (*k, *v)))
        {
            let dst = wire_loc.get(&k).unwrap();
            for (gate, side) in dst {
                match side {
                    Side::Lhs => this_gates.get_mut(gate).unwrap().lhs.replace(v),
                    Side::Rhs => this_gates.get_mut(gate).unwrap().rhs.replace(v),
                };
            }
        }
        // calculate out wire
        let mut new_carry_wires = HashMap::new();
        let mut new_carry_gates = HashMap::new();
        for (name, gate) in this_gates.iter_mut() {
            if let (Some(l), Some(r)) = (gate.lhs, gate.rhs) {
                let new_value = match gate.op {
                    Op::And => l & r,
                    Op::Or => l | r,
                    Op::Xor => l ^ r,
                };
                new_carry_wires.insert(gate.out, new_value);
            } else {
                new_carry_gates.insert(*name, gate.clone());
            }
        }
        // check z45
        if let Some(z) = new_carry_wires.remove("z45") {
            if z == zb[45] {
                z45 = new_changes;
                break;
            } else {
                continue;
            }
        }
        // if valid
        if let Some(z) = new_carry_wires.remove(zk.as_str()) {
            if z == zb[i] {
                println!("is valid");
                deq.push_back(State {
                    i: i + 1,
                    changes: new_changes,
                    to_change: None,
                    carry_gates: new_carry_gates.clone(),
                    carry_wires: new_carry_wires.clone(),
                });
            } else {
                println!("not valid");
            }
        } else {
            println!("keep going");
            deq.push_back(State {
                i,
                changes: new_changes,
                to_change: None,
                carry_gates: new_carry_gates.clone(),
                carry_wires: new_carry_wires.clone(),
            });
        }
    }
    let mut ans = z45.into_iter().flat_map(|x| [x.0, x.1]).collect_vec();
    ans.sort_unstable();
    ans.join(",")
}

type XYZ = (usize, usize, usize);
type VecXYZ = (Vec<usize>, Vec<usize>, Vec<usize>);

fn find_z(wire_val: &WireVal) -> (XYZ, VecXYZ) {
    let mut xs: Vec<(&str, usize)> = wire_val
        .iter()
        .filter(|(k, _)| k.starts_with("x"))
        .map(|(k, v)| (*k, *v))
        .collect();
    let mut ys: Vec<(&str, usize)> = wire_val
        .iter()
        .filter(|(k, _)| k.starts_with("y"))
        .map(|(k, v)| (*k, *v))
        .collect();
    xs.sort_unstable_by(|a, b| b.0.cmp(a.0));
    let xn = xs.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    let x = xn
        .iter()
        .cloned()
        .reduce(|acc, ele| acc << 1 | ele)
        .unwrap();
    ys.sort_unstable_by(|a, b| b.0.cmp(a.0));
    let yn = ys.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    let y = yn
        .iter()
        .cloned()
        .reduce(|acc, ele| acc << 1 | ele)
        .unwrap();
    let z = x + y;
    let zs = format!("{z:b}")
        .chars()
        .rev()
        .map(|c| c.to_digit(2).unwrap() as usize)
        .collect();
    ((x, y, z), (xn, yn, zs))
}
