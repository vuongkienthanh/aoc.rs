use crate::parsing::parse_input;
use fxhash::FxHashMap;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut child_map = FxHashMap::default();

    for (a, b) in input {
        child_map.insert(b, a);
    }

    let mut ans = 0;
    let planets: Vec<&str> = child_map.keys().cloned().collect();

    for mut planet in planets {
        while let Some(child) = child_map.get(&planet) {
            ans += 1;
            planet = *child;
        }
    }

    ans
}
