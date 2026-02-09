use crate::parse;
use crate::part1::find_invalid;
use itertools::Itertools;
use std::cmp::Ordering;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let invalid = find_invalid(&input);
    let mut ans = 0;
    'a: for i in 0..input.len() {
        let mut sum = input[i];
        for j in i + 1..input.len() {
            sum += input[j];
            match sum.cmp(&invalid) {
                Ordering::Equal => {
                    let (min, max) = input[i..=j].into_iter().minmax().into_option().unwrap();
                    ans = min + max;
                    break 'a;
                }
                Ordering::Less => (),
                Ordering::Greater => break,
            }
        }
    }
    ans
}
