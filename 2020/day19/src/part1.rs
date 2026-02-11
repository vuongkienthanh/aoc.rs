use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (map, msg) = parse_input(_input);
    msg.into_iter()
        .filter(|line| map.get(&0).unwrap().is_matched_all(line, &map))
        .count()
}
