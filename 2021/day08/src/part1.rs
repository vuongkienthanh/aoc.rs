use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .flat_map(|(_, a)| a.into_iter())
        .filter(|x| [2, 3, 4, 7].contains(&x.count_ones()))
        .count()
}
