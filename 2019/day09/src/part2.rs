use crate::parsing::parse_input;
use crate::Computer;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut ans = vec![];
    let mut comp = Computer::new(input);
    comp.append_input(2);
    while let Some(output) = comp.long_run() {
        ans.push(output);
    }
    ans.into_iter().next().unwrap() as usize
}
