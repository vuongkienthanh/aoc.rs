use crate::parsing::parse_input;
use crate::{Actor, Map, run_seq};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut map = Map::new();
    let actor :Actor = ((0, 0), 0);
    run_seq(actor, &mut map, &input);
    map.into_values().filter(|x| *x>=1000).count()
}