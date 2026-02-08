use crate::parsing::{Item, parse_input};
use std::collections::VecDeque;

pub fn process(_input: &str) -> String {
    let input = parse_input(_input);
    let mut s: VecDeque<char> = "fbgdceah".chars().collect();
    for item in input.into_iter().rev() {
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
                let removed = s.remove(b).unwrap_unchecked();
                s.insert(a, removed);
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
            Item::RotateLeft(a) => s.rotate_right(a),
            Item::RotateRight(a) => s.rotate_left(a),
            Item::RotateBased(a) => {
                for r in 0..s.len() {
                    let mut maybe = s.clone();
                    maybe.rotate_left(r);

                    unsafe {
                        let first_a_index = maybe
                            .iter()
                            .enumerate()
                            .find_map(|(i, c)| (c == &a).then_some(i))
                            .unwrap_unchecked();
                        maybe.rotate_right(1 + first_a_index);
                        if first_a_index >= 4 {
                            maybe.rotate_right(1);
                        }
                    }
                    if maybe == s {
                        s.rotate_left(r);
                        break;
                    }
                }
            }
        }
    }
    s.into_iter().collect()
}
