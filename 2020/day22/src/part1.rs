use crate::parsing::parse_input;
use std::cmp::Ordering;

pub fn process(_input: &str) -> usize {
    let (mut p1, mut p2) = parse_input(_input);
    while p1.len() > 0 && p2.len() > 0 {
        let a = p1.pop_front().unwrap();
        let b = p2.pop_front().unwrap();
        match a.cmp(&b) {
            Ordering::Equal => panic!(),
            Ordering::Less => {
                p2.push_back(b);
                p2.push_back(a);
            }
            Ordering::Greater => {
                p1.push_back(a);
                p1.push_back(b);
            }
        }
    }
    p1.into_iter()
        .chain(p2)
        .rev()
        .enumerate()
        .map(|(i, n)| n * (i + 1))
        .sum()
}
