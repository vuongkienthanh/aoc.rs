use std::boxed::Box;
use std::collections::HashMap;
use std::collections::VecDeque;
#[derive(PartialEq, Eq, Debug, Clone)]
enum Pulse {
    Low,
    High,
}
impl Default for Pulse {
    fn default() -> Self {
        Pulse::Low
    }
}

trait Module: std::fmt::Debug {
    fn send(&mut self, src: &str, receive: &Pulse) -> Option<(Pulse, Vec<String>)>;
    fn is_backed_to_origin(&self) -> bool;
}

#[derive(Debug)]
struct Flipflop {
    is_on: bool,
    dst: Vec<String>,
}
impl Flipflop {
    fn new(dst: Vec<&str>) -> Self {
        let dst = dst.iter().map(|m| m.to_string()).collect();
        Self { is_on: false, dst }
    }
}
impl Module for Flipflop {
    fn send(&mut self, _src: &str, receive: &Pulse) -> Option<(Pulse, Vec<String>)> {
        if receive == &Pulse::High {
            None
        } else {
            match self.is_on {
                true => {
                    self.is_on = false;
                    Some((Pulse::Low, self.dst.clone()))
                }
                false => {
                    self.is_on = true;
                    Some((Pulse::High, self.dst.clone()))
                }
            }
        }
    }

    fn is_backed_to_origin(&self) -> bool {
        !self.is_on
    }
}
#[derive(Debug)]
struct Conjunction {
    src: HashMap<String, Pulse>,
    dst: Vec<String>,
}
impl Conjunction {
    fn new(src: Vec<&str>, dst: Vec<&str>) -> Self {
        let src = src
            .into_iter()
            .map(|m| (m.to_string(), Pulse::default()))
            .collect();
        let dst = dst.iter().map(|m| m.to_string()).collect();
        Self { src, dst }
    }
}
impl Module for Conjunction {
    fn send(&mut self, src: &str, receive: &Pulse) -> Option<(Pulse, Vec<String>)> {
        *self.src.get_mut(src).unwrap() = receive.clone();
        if self.src.values().all(|p| p == &Pulse::High) {
            Some((Pulse::Low, self.dst.clone()))
        } else {
            Some((Pulse::High, self.dst.clone()))
        }
    }

    fn is_backed_to_origin(&self) -> bool {
        self.src.values().all(|p| p == &Pulse::Low)
    }
}
#[derive(Debug)]
struct Broadcaster {
    dst: Vec<String>,
}
impl Broadcaster {
    fn new(dst: Vec<&str>) -> Self {
        let dst = dst.iter().map(|m| m.to_string()).collect();
        Self { dst }
    }
}
impl Module for Broadcaster {
    fn send(&mut self, _src: &str, receive: &Pulse) -> Option<(Pulse, Vec<String>)> {
        Some((receive.clone(), self.dst.clone()))
    }

    fn is_backed_to_origin(&self) -> bool {
        true
    }
}
fn parse_input<'a>(input: &str) -> HashMap<String, Box<dyn Module>> {
    let conjunctions: Vec<&str> = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap().0)
        .filter(|src| src.starts_with('&'))
        .map(|src| &src[1..])
        .collect();
    let mut conjunction_src: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (src, dst) = line.split_once(" -> ").unwrap();
        for m in dst.split(", ").filter(|m| conjunctions.contains(m)) {
            let name = if src.starts_with(['%', '&']) {
                &src[1..]
            } else {
                src
            };
            conjunction_src.entry(m).or_default().push(name)
        }
    }
    input
        .lines()
        .map(|line| {
            let (mut src, dst) = line.split_once(" -> ").unwrap();
            let dst = dst.split(", ").collect::<Vec<&str>>();
            let module: Box<dyn Module> = if src == "broadcaster" {
                Box::new(Broadcaster::new(dst))
            } else if src.starts_with('%') {
                src = &src[1..];
                Box::new(Flipflop::new(dst))
            } else if src.starts_with('&') {
                src = &src[1..];
                let src = conjunction_src.remove(src).unwrap();
                Box::new(Conjunction::new(src, dst))
            } else {
                panic!("unknown module")
            };
            (src.to_string(), module)
        })
        .collect()
}

pub fn process(input: &'static str) -> usize {
    let mut parsed = parse_input(input);
    let mut loop_len = 0;
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for i in 0..1000 {
        let mut stack: VecDeque<(String, Pulse, String)> =
            VecDeque::from([("button".to_string(), Pulse::Low, "broadcaster".to_string())]);
        low_pulses += 1;
        while let Some((src, pulse, dst)) = stack.pop_front() {
            // dbg!((&src, &pulse, &dst));
            if let Some(module) = parsed.get_mut(&dst) {
                if let Some((new_pulse, new_dsts)) = module.send(&src, &pulse) {
                    for new_dst in new_dsts.into_iter() {
                        stack.push_back((dst.clone(), new_pulse.clone(), new_dst));
                        match new_pulse {
                            Pulse::Low => low_pulses += 1,
                            Pulse::High => high_pulses += 1,
                        }
                    }
                }
            }
        }
        if parsed.values().all(|m| m.is_backed_to_origin()) {
            loop_len = i + 1;
            break;
        }
    }
    let remaining = match loop_len {
        0 => 0,
        l => {
            let number_of_loops = 1000usize.div_euclid(loop_len);
            low_pulses *= number_of_loops;
            high_pulses *= number_of_loops;
            1000usize.rem_euclid(l)
        }
    };

    for _ in 0..remaining {
        let mut stack: VecDeque<(String, Pulse, String)> =
            VecDeque::from([("button".to_string(), Pulse::Low, "broadcaster".to_string())]);
        low_pulses += 1;
        while let Some((src, pulse, dst)) = stack.pop_front() {
            // dbg!((&src, &pulse, &dst));
            if let Some(module) = parsed.get_mut(&dst) {
                if let Some((new_pulse, new_dsts)) = module.send(&src, &pulse) {
                    for new_dst in new_dsts.into_iter() {
                        stack.push_back((dst.clone(), new_pulse.clone(), new_dst));
                        match new_pulse {
                            Pulse::Low => low_pulses += 1,
                            Pulse::High => high_pulses += 1,
                        }
                    }
                }
            }
        }
    }
    // dbg!(&parsed);
    // dbg!(&parsed.values().all(|m| m.is_backed_to_origin()));
    // dbg!(loop_len);
    low_pulses * high_pulses
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_1a() {
        let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
        assert_eq!(process(input), 32000000);
    }
    #[test]
    fn test_process_1b() {
        let input = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;
        assert_eq!(process(input), 11687500);
    }
}
