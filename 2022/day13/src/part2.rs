use crate::parsing::{parse_input, parse_value};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let (_, two) = parse_value("[[2]]").unwrap();
    let (_, six) = parse_value("[[6]]").unwrap();
    let mut v: Vec<_> = input
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .chain([two.clone(), six.clone()])
        .collect();
    v.sort_unstable();
    let a = v
        .iter()
        .enumerate()
        .find_map(|(i, x)| (*x == two).then_some(i + 1))
        .unwrap();
    let b = v
        .into_iter()
        .enumerate()
        .skip(a)
        .find_map(|(i, x)| (x == six).then_some(i + 1))
        .unwrap();
    a * b
}
