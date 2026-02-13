use crate::parsing::parse_input;
use fxhash::FxHashSet;
use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    let (p1, p2) = parse_input(_input);
    let (p1, p2) = game(p1, p2);
    p1.into_iter()
        .chain(p2)
        .rev()
        .enumerate()
        .map(|(i, n)| n * (i + 1))
        .sum()
}

fn game(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut seen = FxHashSet::default();
    while p1.len() > 0 && p2.len() > 0 {
        if seen.insert((p1.clone(), p2.clone())) {
            let a = p1.pop_front().unwrap();
            let b = p2.pop_front().unwrap();
            let p1_win = if a <= p1.len() && b <= p2.len() {
                let new_p1 = p1.iter().take(a).cloned().collect();
                let new_p2 = p2.iter().take(b).cloned().collect();
                recursive_game(new_p1, new_p2)
            } else {
                a > b
            };
            if p1_win {
                p1.push_back(a);
                p1.push_back(b);
            } else {
                p2.push_back(b);
                p2.push_back(a);
            }
        } else {
            break;
        }
    }
    (p1, p2)
}
fn recursive_game(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> bool {
    let mut seen = FxHashSet::default();
    let mut is_seen = false;
    while p1.len() > 0 && p2.len() > 0 {
        if seen.insert((p1.clone(), p2.clone())) {
            let a = p1.pop_front().unwrap();
            let b = p2.pop_front().unwrap();
            let p1_win = if a <= p1.len() && b <= p2.len() {
                let new_p1 = p1.iter().take(a).cloned().collect();
                let new_p2 = p2.iter().take(b).cloned().collect();
                recursive_game(new_p1, new_p2)
            } else {
                a > b
            };
            if p1_win {
                p1.push_back(a);
                p1.push_back(b);
            } else {
                p2.push_back(b);
                p2.push_back(a);
            }
        } else {
            is_seen = true;
            break;
        }
    }
    is_seen || p1.len() > 0
}
