use crate::parse;

pub fn process(_input: &str) -> u16 {
    let mut input = parse(_input);
    input.sort_unstable();
    input
        .windows(2)
        .find_map(|v| (v[1] - v[0] == 2).then_some(v[0] + 1))
        .unwrap()
}
