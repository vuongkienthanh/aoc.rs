use crate::{is_closing_char, matching_closing_char};

pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let mut stack = vec![];
            let mut score = 0;
            for c in line.chars() {
                if is_closing_char(c) {
                    let last = stack.pop().unwrap();
                    if matching_closing_char(last) != c {
                        score = match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!(),
                        };
                        break;
                    }
                } else {
                    stack.push(c);
                }
            }
            score
        })
        .sum()
}
