use intcode::{Computer, RunResult, parse};

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut ans = vec![];
    let mut comp = Computer::new(input);
    comp.input(1);
    loop {
        match comp.long_run() {
            RunResult::WaitingInput => panic!("should be enough input"),
            RunResult::Halt => break,
            RunResult::Output(output) => ans.push(output),
        };
    }
    ans.into_iter().next().unwrap() as usize
}
