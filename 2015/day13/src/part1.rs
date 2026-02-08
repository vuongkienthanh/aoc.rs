use crate::parsing::parse_input;
use crate::{build_all_combinations, calculate_happiness};

pub fn process(_input: &str) -> isize {
    let graph = parse_input(_input);

    let names = graph.keys().cloned().collect::<Vec<_>>();

    build_all_combinations(names)
        .into_iter()
        .map(|v| calculate_happiness(&v, &graph))
        .max()
        .unwrap()
}
