use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .filter(|(low, high, c, line)| {
            let x = line.chars().filter(|x| x == c).count();
            x >= *low && x <= *high
        })
        .count()
}
