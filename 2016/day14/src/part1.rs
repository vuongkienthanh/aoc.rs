use crate::{find_five, find_three};
use fxhash::{FxHashMap as Map, FxHashSet as Set};

pub fn process(_input: &str) -> usize {
    let mut possible_keys: Map<char, Set<usize>> = Map::default();
    let mut count = 0;
    for i in 0.. {
        let input = format!("{_input}{i}");
        let hash = format!("{:x}", md5::compute(input));
        if let Some(c) = find_three(&hash) {
            possible_keys.entry(c).or_default().insert(i);
        }
        let mut found_keys = vec![];
        for c in find_five(&hash) {
            let possible_keys_for_this_char = possible_keys.get(&c).cloned().unwrap();
            for pk in possible_keys_for_this_char {
                if (i.saturating_sub(1000)..i).contains(&pk) {
                    possible_keys.get_mut(&c).unwrap().remove(&pk);
                    found_keys.push(pk);
                }
            }
        }
        found_keys.sort_unstable();
        for k in found_keys {
            count += 1;
            if count == 64 {
                return k;
            }
        }
    }
    panic!("should have an answer")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("abc", 22728)]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
