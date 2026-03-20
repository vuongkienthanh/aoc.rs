use crate::parsing::parse_input;

use fxhash::FxHashMap;

#[derive(Debug, Clone)]
struct Node {
    next: usize,
    prev: usize,
}

fn build_hm(len: usize) -> FxHashMap<usize, Node> {
    let mut hm = FxHashMap::default();
    hm.insert(
        0,
        Node {
            prev: len - 1,
            next: 1,
        },
    );
    hm.insert(
        len - 1,
        Node {
            prev: len - 2,
            next: 0,
        },
    );
    for i in 1..len - 1 {
        hm.insert(
            i,
            Node {
                prev: i - 1,
                next: i + 1,
            },
        );
    }
    hm
}
fn rebuild_vec(input: &[isize], hm: &FxHashMap<usize, Node>) -> Vec<isize> {
    let mut ans = vec![];
    let mut i = 0;
    for _ in 0..hm.len() {
        ans.push(input[i]);
        i = hm.get(&i).unwrap().next;
    }
    ans
}

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    let mut hm = build_hm(input.len());
    for i in 0..input.len() {
        let v = input[i];
        if v == 0 {
            continue;
        }
        let Node { mut prev, mut next } = hm.get(&i).cloned().unwrap();
        hm.get_mut(&prev).unwrap().next = next;
        hm.get_mut(&next).unwrap().prev = prev;
        if v >= 0 {
            for _ in 0..v - 1 {
                let n = hm.get(&next).unwrap();
                next = n.next;
            }
            let left = next;
            let right = hm.get(&next).unwrap().next;
            hm.get_mut(&left).unwrap().next = i;
            hm.get_mut(&right).unwrap().prev = i;
            hm.get_mut(&i).unwrap().prev = left;
            hm.get_mut(&i).unwrap().next = right;
        } else {
            for _ in v + 1..0 {
                let n = hm.get(&prev).unwrap();
                prev = n.prev;
            }
            let right = prev;
            let left = hm.get(&prev).unwrap().prev;
            hm.get_mut(&left).unwrap().next = i;
            hm.get_mut(&right).unwrap().prev = i;
            hm.get_mut(&i).unwrap().prev = left;
            hm.get_mut(&i).unwrap().next = right;
        }
        let v = rebuild_vec(&input, &hm);
    }
    let mut ans = 0;
    let mut current = input
        .iter()
        .enumerate()
        .find_map(|(i, x)| (*x == 0).then_some(i))
        .unwrap();
    for _ in 0..1000 {
        current = hm.get(&current).unwrap().next;
    }
    ans += input[current];
    for _ in 0..1000 {
        current = hm.get(&current).unwrap().next;
    }
    ans += input[current];
    for _ in 0..1000 {
        current = hm.get(&current).unwrap().next;
    }
    ans += input[current];

    ans
}