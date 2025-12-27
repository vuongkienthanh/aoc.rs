use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .map(|p| p.a.iter().map(|c| c.pow(2u32)).sum::<isize>())
        .enumerate()
        .min_by_key(|(_, p)| *p)
        .map(|(i, _)| i)
        .unwrap()
}
