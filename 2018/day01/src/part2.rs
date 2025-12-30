use crate::parsing::parse_input;
use fxhash::FxHashSet as Set;

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    let mut seen: Set<isize> = Set::default();
    let mut result = 0;
    seen.insert(result);
    for freq in input.into_iter().cycle() {
        result += freq;
        if !seen.insert(result) {
            return result;
        }
    }
    panic!("should have an answer")
}
