use crate::parse;
use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    find_invalid(&input)
}

pub fn find_invalid(input: &[usize]) -> usize {
    let mut invalid = 0;
    let mut preamble = get_preamble(&input);
    for i in 25..input.len() {
        let n = input[i];
        if is_valid(n, &preamble) {
            update_preamble(n, &mut preamble);
        } else {
            invalid = n;
            break;
        }
    }
    invalid
}

fn get_preamble(input: &[usize]) -> VecDeque<(usize, Vec<usize>)> {
    let mut preamble = VecDeque::new();
    for i in 0..25 {
        let mut line = vec![];
        for j in i + 1..25 {
            line.push(input[i] + input[j]);
        }
        preamble.push_back((input[i], line));
    }
    preamble
}

fn is_valid(n: usize, preamble: &VecDeque<(usize, Vec<usize>)>) -> bool {
    preamble.iter().any(|(_, v)| v.contains(&n))
}

fn update_preamble(n: usize, preamble: &mut VecDeque<(usize, Vec<usize>)>) {
    let _ = preamble.pop_front().unwrap();
    for (i, v) in preamble.iter_mut() {
        v.push(*i + n);
    }
    preamble.push_back((n, vec![]));
}
