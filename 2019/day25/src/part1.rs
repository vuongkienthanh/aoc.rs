use crate::{parse_block, parse_output};
use fxhash::FxHashSet;
use intcode::{Computer, RunResult, parse};

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut comp = Computer::new(input);
    let mut seen: FxHashSet<State> = FxHashSet::default();
    let mut current = vec![];
    loop {
        match comp.long_run() {
            RunResult::Halt => panic!("should not halt"),
            RunResult::WaitingInput => break,
            RunResult::Output(x) => output.push(x as u8 as char),
        }
    }
    let x = parse_output(&output);
    println!("{x:?}");
    0
}

struct State<'a> {
    comp: Computer,
    loc: &'a str,
    items: Vec<&'a str>,
}
