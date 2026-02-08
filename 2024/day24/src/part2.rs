use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

impl From<&str> for GateType {
    fn from(s: &str) -> Self {
        match s {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => panic!("Unknown gate type: {}", s),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    q: &'a str,
    t: GateType,
}

impl<'a> Gate<'a> {
    fn new(a: &'a str, b: &'a str, t: GateType, q: &'a str) -> Self {
        Gate { a, b, t, q }
    }

    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, gate) = map(
            tuple((
                alphanumeric1,
                tag(" "),
                alt((tag("AND"), tag("OR"), tag("XOR"))),
                tag(" "),
                alphanumeric1,
                tag(" -> "),
                alphanumeric1,
            )),
            |(a, _, t, _, b, _, s)| Gate::new(a, b, t.into(), s),
        )(input)?;
        Ok((input, gate))
    }

    fn run(&self, a: bool, b: bool) -> bool {
        match self.t {
            GateType::And => a && b,
            GateType::Or => a || b,
            GateType::Xor => a ^ b,
        }
    }

    fn has_input(&self, wire: &str) -> bool {
        self.a == wire || self.b == wire
    }

    fn has_output(&self, wire: &str) -> bool {
        self.q == wire
    }

    fn is_half_adder_input(&self) -> bool {
        self.a == "x00" || self.b == "x00"
    }

    fn is_input(&self) -> bool {
        self.a.starts_with("x")
            || self.b.starts_with("x")
            || self.a.starts_with("y")
            || self.b.starts_with("y")
    }

    fn is_output(&self) -> bool {
        self.q.starts_with("z")
    }

    fn is_type(&self, t: GateType) -> bool {
        self.t == t
    }
}

#[derive(Debug)]
struct Wires<'a> {
    wires: HashMap<&'a str, bool>,
}

impl<'a> Wires<'a> {
    fn parse_bool(input: &'a str) -> IResult<&'a str, bool> {
        let (input, b) = alt((tag("0"), tag("1")))(input)?;
        Ok((input, b == "1"))
    }

    fn parse_wire(input: &'a str) -> IResult<&'a str, (&'a str, bool)> {
        let (input, (name, _, state)) =
            tuple((alphanumeric1, tag(": "), Wires::parse_bool))(input)?;
        Ok((input, (name, state)))
    }

    fn parse_all(input: &'a str) -> IResult<&'a str, Self> {
        let (input, wires) = separated_list1(tag("\n"), Wires::parse_wire)(input)?;
        let wires = wires.into_iter().collect();
        Ok((input, Self { wires }))
    }

    fn get(&self, wire: &'a str) -> Option<bool> {
        self.wires.get(wire).copied()
    }

    fn get_inputs(&self, a: &'a str, b: &'a str) -> Option<(bool, bool)> {
        let a = self.get(a)?;
        let b = self.get(b)?;
        Some((a, b))
    }

    fn set(&mut self, wire: &'a str, value: bool) {
        self.wires.insert(wire, value);
    }

    fn with_prefix(&self, prefix: &str) -> Vec<(&'a str, bool)> {
        self.wires
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(&k, &v)| (k, v))
            .collect()
    }
}

#[derive(Debug)]
struct Device<'a> {
    gates: Vec<Gate<'a>>,
    wires: Wires<'a>,
}

impl<'a> Device<'a> {
    fn new(input: &'a str) -> IResult<&'a str, Self> {
        let (input, wires) = Wires::parse_all(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, gates) = separated_list1(tag("\n"), Gate::parse)(input)?;
        Ok((input, Device { gates, wires }))
    }

    fn run(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for gate in &self.gates {
                if let Some((a, b)) = self.wires.get_inputs(gate.a, gate.b) {
                    // If we already have a value for this wire, skip it.
                    if let Some(_) = self.wires.get(gate.q) {
                        continue;
                    }
                    self.wires.set(gate.q, gate.run(a, b));
                    changed = true;
                }
            }
        }
    }

    fn z_as_usize(&self) -> usize {
        let mut wires = self.wires.with_prefix("z");
        wires.sort_by(|a, b| b.cmp(a));
        wires.into_iter().fold(0_usize, |mut acc, (_, z)| {
            acc = acc << 1;
            if z {
                acc |= 1;
            }
            acc
        })
    }
}

pub fn process(input: &str) -> String {
    let (_, mut device) = Device::new(input).unwrap();

    // Track bad wires.
    let mut bad_wires = vec![];

    let input_xor_gates = device
        .gates
        .iter()
        .filter(|gate| gate.is_input() && gate.is_type(GateType::Xor))
        .cloned()
        .collect::<Vec<_>>();

    // First check all the input xor gates.
    for gate in &input_xor_gates {
        // The initial half adder gate should have a z00 output. No other input
        // gates should have z00.
        match (gate.is_half_adder_input(), gate.has_output("z00")) {
            (true, true) => continue,
            (true, false) => bad_wires.push(gate.q),
            (false, true) => bad_wires.push(gate.q),
            (false, false) => continue,
        }
        // All other input gates should not have an output because it needs to
        // be XOR with the previous carry wire.
        if gate.is_output() {
            bad_wires.push(gate.q);
        }
    }

    let intermediate_xor_gates = device
        .gates
        .iter()
        .filter(|gate| !gate.is_input() && gate.is_type(GateType::Xor))
        .cloned()
        .collect::<Vec<_>>();

    // All other XOR gates should be internal to the full adders and produce an
    // output.
    intermediate_xor_gates.iter().for_each(|gate| {
        if !gate.is_output() {
            bad_wires.push(gate.q);
        }
    });

    let output_gates = device
        .gates
        .iter()
        .filter(|gate| gate.is_output())
        .cloned()
        .collect::<Vec<_>>();

    // All output gates but the last one should come from a carry wire and the
    // XOR of the input wires. The last wire is just the last carry wire of the last
    // full adder.
    let last_z_wire = format!(
        "z{:02}",
        device.gates.iter().filter(|gate| gate.is_output()).count() - 1
    );
    for gate in &output_gates {
        if gate.has_output(&last_z_wire) {
            if !gate.is_type(GateType::Or) {
                bad_wires.push(gate.q);
            }
            continue;
        } else if !gate.is_type(GateType::Xor) {
            bad_wires.push(gate.q);
        }
    }

    // Gather all the variables from the input_gates
    let mut check = vec![];
    for gate in &input_xor_gates {
        // No need to check the first half adder or any bad wires we've already found.
        if bad_wires.contains(&gate.q) || gate.has_output("z00") {
            continue;
        }

        // Look for intermediate xor gates that don't have the output of an input XOR gate.
        if intermediate_xor_gates
            .iter()
            .filter(|g| g.has_input(gate.q))
            .count()
            == 0
        {
            // They are bad wires and we need to check them later.
            bad_wires.push(gate.q);
            check.push(gate);
        }
    }

    for gate in check {
        let expected = format!("z{}", &gate.a[1..]);
        let m = intermediate_xor_gates
            .iter()
            .find(|gate| gate.has_output(&expected))
            .unwrap();

        let o = device
            .gates
            .iter()
            .find(|gate| gate.is_type(GateType::Or) && (gate.q == m.a || gate.q == m.b))
            .unwrap();

        if m.a != o.q {
            bad_wires.push(m.a);
        }
        if m.b != o.q {
            bad_wires.push(m.b);
        }
    }

    bad_wires.sort();
    bad_wires.join(",")
}
