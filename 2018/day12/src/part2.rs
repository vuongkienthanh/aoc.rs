use crate::parsing::parse_input;
use crate::part1::run;

pub fn process(_input: &str) -> usize {
    let (plants, map) = parse_input(_input);
    // after 100 generations
    // the vec converges to 11 pairs of 1 [0,0,1,1,0,....]
    // [0,1,1,0,0] can only generate to [0,0,1,1,0]
    // (one step further with no new variation)
    // it means each generation increases every value by 1
    run(plants, map, 100) + 22 * (50_000_000_000 - 100)
}
