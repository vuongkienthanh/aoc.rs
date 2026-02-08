pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq,Clone)]
enum PredicationResult {
    Accept,
    Reject,
    Refer(String),
}
impl PredicationResult {
    fn new(code: &str) -> Self {
        match code {
            "A" => Self::Accept,
            "R" => Self::Reject,
            s => Self::Refer(s.to_string()),
        }
    }
}
#[derive(Debug, Clone)]
enum MachinePartAttribute {
    X,
    M,
    A,
    S,
}
impl MachinePartAttribute {
    fn new(code: char) -> Self {
        match code {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("not in xmas"),
        }
    }
}

#[derive(Debug, Clone)]
enum CompareSign {
    G,
    L,
}
impl CompareSign {
    fn new(code: char) -> Self {
        match code {
            '<' => Self::L,
            '>' => Self::G,
            _ => panic!("not in < or >"),
        }
    }
}
