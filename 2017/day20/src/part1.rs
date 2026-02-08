use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .map(|p| p.a.iter().map(|c| c.unsigned_abs()).sum::<usize>())
        .enumerate()
        .min_by_key(|(_, p)| *p)
        .map(|(i, _)| i)
        .unwrap()
}
