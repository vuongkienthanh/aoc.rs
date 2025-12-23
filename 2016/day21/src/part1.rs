use crate::parsing::{Item, parse_input};
use std::collections::VecDeque;

pub fn process(_input: &str) -> String {
    let input = parse_input(_input);
    let mut s: VecDeque<char> = "abcdefgh".chars().collect();

    for item in input {
        match item {
            Item::SwapPosition(a, b) => s.swap(a, b),
            Item::SwapLetter(a, b) => {
                for i in 0..s.len() {
                    unsafe {
                        if s.get(i).unwrap_unchecked() == &a {
                            *s.get_mut(i).unwrap_unchecked() = b;
                            continue;
                        }
                        if s.get(i).unwrap_unchecked() == &b {
                            *s.get_mut(i).unwrap_unchecked() = a;
                            continue;
                        }
                    }
                }
            }
            Item::MovePosition(a, b) => unsafe {
                let removed = s.remove(a).unwrap_unchecked();
                s.insert(b, removed);
            },
            Item::Reverse(a, b) => {
                for i in 0.. {
                    let left = a + i;
                    let right = b - i;
                    if left >= right {
                        break;
                    }
                    s.swap(left, right);
                }
            }
            Item::RotateLeft(a) => s.rotate_left(a),
            Item::RotateRight(a) => s.rotate_right(a),
            Item::RotateBased(a) => unsafe {
                let first_a_index = s
                    .iter()
                    .enumerate()
                    .find_map(|(i, c)| (c == &a).then_some(i))
                    .unwrap_unchecked();
                s.rotate_right(1 + first_a_index);
                if first_a_index >= 4 {
                    s.rotate_right(1);
                }
            },
        }
    }

    s.into_iter().collect()
}
