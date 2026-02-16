use crate::{is_closing_char, matching_closing_char};

pub fn process(_input: &str) -> usize {
    let mut scores: Vec<usize> = _input
        .lines()
        .filter_map(|line| {
            let mut stack = vec![];
            let mut is_corrupted = false;
            for c in line.chars() {
                if is_closing_char(c) {
                    let last = stack.pop().unwrap();
                    if matching_closing_char(last) != c {
                        is_corrupted = true;
                        break;
                    }
                } else {
                    stack.push(c);
                }
            }
            (!is_corrupted).then_some(
                stack
                    .into_iter()
                    .map(|c| match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!(),
                    })
                    .rfold(0, |acc, ele| acc * 5 + ele),
            )
        })
        .collect();
    scores.sort_unstable();
    let i = scores.len() / 2;
    scores[i]
}
