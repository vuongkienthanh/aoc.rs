use crate::parsing::{Mapping, parse_input};
use crate::total_distance;
use itertools::Itertools;
use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    let mapping = parse_input(_input);
    mapping
        .keys()
        .copied()
        .combinations(2)
        .map(|comb| {
            let (loc1, loc2) = (comb[0], comb[1]);
            max_distance(loc1, loc2, &mapping)
        })
        .max()
        .expect("should have an answer")
}
fn max_distance<'a>(loc1: &'a str, loc2: &'a str, mapping: &Mapping<'a>) -> usize {
    let mut routes = VecDeque::from([vec![loc1]]);
    let mut max = usize::MIN;
    while let Some(v) = routes.pop_front() {
        let last_loc = v.last().unwrap();
        for next_loc in mapping.get(last_loc).unwrap().keys() {
            if !v.contains(next_loc) {
                if next_loc == &loc2 {
                    if v.len() + 1 == mapping.len() {
                        let mut new_v = v.clone();
                        new_v.push(next_loc);
                        max = max.max(total_distance(new_v, mapping));
                    }
                } else {
                    let mut new_v = v.clone();
                    new_v.push(next_loc);
                    routes.push_back(new_v);
                }
            }
        }
    }
    max
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(
        r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#,
        982
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
