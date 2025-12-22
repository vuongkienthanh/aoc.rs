use crate::parsing::parse_input;
use crate::{build_all_combinations, calculate_happiness};

pub fn process(_input: &str) -> isize {
    let mut graph = parse_input(_input);

    let mut names = graph.keys().cloned().collect::<Vec<_>>();

    let me = "me";
    for n in &names {
        graph.entry(me).or_default().insert(n, 0);
        graph.entry(n).or_default().insert(me, 0);
    }
    names.push(me);

    build_all_combinations(names)
        .into_iter()
        .map(|v| calculate_happiness(&v, &graph))
        .max()
        .unwrap()
}
