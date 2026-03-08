use crate::parsing::parse_input;
use crate::{radius_map, range_at};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let map = radius_map(input);
    let (mut r, mut c) = (0, 0);
    for row in 0..=4000000 {
        let rg = range_at(&map, row);
        if rg.len() == 2 {
            r = row;
            c = rg[0].1 + 1;
            println!("row={r} col={c}");
            break;
        }
    }
    (c * 4000000 + r) as usize
}
