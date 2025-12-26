use crate::parsing::parse_input;
use fxhash::{FxHashMap as Map, FxHashSet as Set};

pub fn process(_input: &str) -> usize {
    let mut map = parse_input(_input);
    let mut groups = 0;
    while !map.is_empty() {
        let group = find_a_group(&map);
        groups += 1;
        for id in group {
            map.remove(&id);
        }
    }
    groups
}

fn find_a_group(map: &Map<usize, Vec<usize>>) -> Set<usize> {
    let mut seen: Set<usize> = Set::default();
    let first = map.keys().next().cloned().unwrap();
    let mut current = vec![first];

    while !current.is_empty() {
        let mut new = vec![];
        for id in current {
            if seen.insert(id) {
                for next_id in map.get(&id).cloned().unwrap() {
                    new.push(next_id);
                }
            }
        }

        current = new;
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 2);
    }
}
