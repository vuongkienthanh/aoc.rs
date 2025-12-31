use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    ('a'..='z').map(|c| react(_input, c).len()).min().unwrap()
}
fn react(input: &str, rm: char) -> VecDeque<u8> {
    let mut left: VecDeque<u8> = VecDeque::new();
    let mut right: VecDeque<u8> = VecDeque::new();
    right.extend(input.bytes());
    let (rm0, rm1) = (rm as u8, rm as u8 - 32);

    while let Some(r) = right.pop_front() {
        if r == rm0 || r == rm1 {
            continue;
        }
        if let Some(l) = left.pop_back() {
            if l.abs_diff(r) != 32 {
                left.push_back(l);
                left.push_back(r);
            }
        } else {
            left.push_back(r);
        }
    }

    left
}
