use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .filter(|((a, b), (c, d))| {
            (a <= c && b >= d) || (c <= a && d >= b) || (a >= c && a <= d) || (b >= c && b <= d)
        })
        .count()
}