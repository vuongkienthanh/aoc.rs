use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    input
        .into_iter()
        .filter(|(i, j, c, line)| {
            let x = line.chars().nth(*i - 1).unwrap() == *c;
            let y = line.chars().nth(*j - 1).unwrap() == *c;
            x ^ y
        })
        .count()
}
