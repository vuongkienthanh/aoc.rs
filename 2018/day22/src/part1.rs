use crate::parsing::parse_input;
use crate::{Cache, get_type};

pub fn process(_input: &str) -> usize {
    let (depth, target) = parse_input(_input);
    let mut cache = Cache::new();
    let mut ans = 0;

    for y in 0..target.1 + 1 {
        for x in 0..target.0 + 1 {
            ans += get_type((x, y), depth, target, &mut cache);
        }
    }
    ans
}
