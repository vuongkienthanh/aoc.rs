use crate::parse;
pub fn process(_input: &str) -> usize {
    let (target, buses) = parse(_input);
    buses
        .into_iter()
        .flatten()
        .map(|x| (x, (target / x + 1) * x))
        .min_by_key(|(_, x)| *x)
        .map(|(a, b)| a * (b - target))
        .unwrap()
}
