use crate::parsing::parse_input;
use aoc_helper::range::merge;

pub fn process(_input: &str) -> u32 {
    let input = parse_input(_input);
    let mut input = merge(input);
    input.sort_unstable_by_key(|x| x.0);

    let mut ans = 0;
    // from 0 to first
    ans += input[0].0;
    // from last to u32::MAX
    ans += u32::MAX - input.last().unwrap().1;

    for pair in input.windows(2) {
        let left_end = pair[0].1;
        let right_start = pair[1].0;
        let diff = right_start - left_end - 1;
        ans += diff;
    }
    ans
}
