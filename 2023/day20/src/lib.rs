pub mod part1;
pub mod part2;
use std::collections::HashMap;
#[derive(PartialEq, Eq, Debug, Clone)]
enum Pulse {
    Low,
    High,
}

enum Module<'a> {
    Flipflop(Flipflop<'a>),
    Conjunction(Conjunction<'a>),
    Broadcaster(Broadcaster<'a>),
}
impl<'a> Module<'a> {
    fn inner(&self) -> &dyn ModuleFunction {
        match self {
            Module::Flipflop(i) => i,
            Module::Conjunction(i) => i,
            Module::Broadcaster(i) => i,
        }
    }
    fn inner_mut(&mut self) -> &mut dyn ModuleFunction {
        match self {
            Module::Flipflop(i) => i,
            Module::Conjunction(i) => i,
            Module::Broadcaster(i) => i,
        }
    }
}

trait ModuleFunction: std::fmt::Debug {
    fn send(&mut self, src: &str, receive: &Pulse) -> Option<(Pulse, Vec<String>)>;
    fn is_backed_to_origin(&self) -> bool;
    fn dst(&self) -> Vec<String>;
    fn src(&self) -> Option<Vec<String>> {
        None
    }
}

#[derive(Debug)]
struct Flipflop<'a> {
    is_on: bool,
    dst: Vec<&'a str>,
}
impl<'a> Flipflop<'a> {
    fn new(dst: Vec<&'a str>) -> Self {
        Self { is_on: false, dst }
    }
}
impl<'a> ModuleFunction for Flipflop<'a> {
    fn send(&mut self, _src: &str, receive: &Pulse) -> Option<(Pulse, Vec<String>)> {
        if receive == &Pulse::High {
            None
        } else {
            match self.is_on {
                true => {
                    self.is_on = false;
                    Some((Pulse::Low, self.dst()))
                }
                false => {
                    self.is_on = true;
                    Some((Pulse::High, self.dst()))
                }
            }
        }
    }

    fn is_backed_to_origin(&self) -> bool {
        !self.is_on
    }

    fn dst(&self) -> Vec<String> {
        self.dst.iter().map(|d| d.to_string()).collect()
    }
}
#[derive(Debug)]
struct Conjunction<'a> {
    src: HashMap<&'a str, Pulse>,
    dst: Vec<&'a str>,
}
impl<'a> Conjunction<'a> {
    fn new(src: Vec<&'a str>, dst: Vec<&'a str>) -> Self {
        let src = src
            .into_iter()
            .map(|m| (m, Pulse::Low))
            .collect();
        Self { src, dst }
    }
}
impl<'a> ModuleFunction for Conjunction<'a> {
    fn send(&mut self, src: &str, receive: &Pulse) -> Option<(Pulse, Vec<String>)> {
        *self.src.get_mut(src).unwrap() = receive.clone();
        if self.src.values().all(|p| p == &Pulse::High) {
            Some((Pulse::Low, self.dst()))
        } else {
            Some((Pulse::High, self.dst()))
        }
    }

    fn is_backed_to_origin(&self) -> bool {
        self.src.values().all(|p| p == &Pulse::Low)
    }

    fn dst(&self) -> Vec<String> {
        self.dst.iter().map(|d| d.to_string()).collect()
    }

    fn src(&self) -> Option<Vec<String>> {
        Some(self.src.keys().map(|d| d.to_string()).collect::<Vec<_>>())
    }
}
#[derive(Debug)]
struct Broadcaster<'a> {
    dst: Vec<&'a str>,
}
impl<'a> Broadcaster<'a> {
    fn new(dst: Vec<&'a str>) -> Self {
        Self { dst }
    }
}
impl<'a> ModuleFunction for Broadcaster<'a> {
    fn send(&mut self, _src: &str, receive: &Pulse) -> Option<(Pulse, Vec<String>)> {
        Some((receive.clone(), self.dst()))
    }

    fn is_backed_to_origin(&self) -> bool {
        true
    }
    fn dst(&self) -> Vec<String> {
        self.dst.iter().map(|d| d.to_string()).collect()
    }
}
fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Module<'a>> {
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
            let (mut module_name, dst) = line.split_once(" -> ").unwrap();
            let dst = dst.split(", ").collect::<Vec<&str>>();
            let module: Module = if module_name == "broadcaster" {
                Module::Broadcaster(Broadcaster::new(dst))
            } else if module_name.starts_with('%') {
                module_name = &module_name[1..];
                Module::Flipflop(Flipflop::new(dst))
            } else if module_name.starts_with('&') {
                module_name = &module_name[1..];
                let src = conjunction_src.remove(module_name).unwrap();
                Module::Conjunction(Conjunction::new(src, dst))
            } else {
                panic!("unknown module")
            };
            (module_name, module)
        })
        .collect()
}
