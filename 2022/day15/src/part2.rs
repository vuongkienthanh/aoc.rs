use crate::parsing::parse_input;
use crate::{radius_map, range_at};
use rayon::prelude::*;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let map = radius_map(input);
    (0..=4000000)
        .into_par_iter()
        .find_map_any(|row| {
            let rg = range_at(&map, row);
            (rg.len() == 2).then_some({
                let col = rg[0].1 + 1;
                (col * 4000000 + row) as usize
            })
        })
        .unwrap()
}
