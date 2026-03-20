use crate::parsing::parse_input;
use crate::{Node, build_hm};

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    let mut hm = build_hm(input.len());
    let tlen = input.len() as isize - 1;
    for (i, v) in input.iter().enumerate() {
        let v = input[i];
        let optimized_v = [(v % tlen + tlen) % tlen, (v % tlen + tlen) % tlen - tlen]
            .into_iter()
            .min_by_key(|x| x.abs())
            .unwrap();
        if optimized_v == 0 {
            continue;
        }
        let node = hm.get(i).unwrap();
        let mut prev = node.prev;
        let mut next = node.next;
        hm.get_mut(prev).unwrap().next = next;
        hm.get_mut(next).unwrap().prev = prev;
        if optimized_v > 0 {
            for _ in 0..optimized_v - 1 {
                let n = hm.get(next).unwrap();
                next = n.next;
            }
            let left = next;
            let right = hm.get(next).unwrap().next;
            hm.get_mut(left).unwrap().next = i;
            hm.get_mut(right).unwrap().prev = i;
            *hm.get_mut(i).unwrap() = Node {
                prev: left,
                next: right,
            };
        } else {
            for _ in optimized_v + 1..0 {
                let n = hm.get(prev).unwrap();
                prev = n.prev;
            }
            let right = prev;
            let left = hm.get(prev).unwrap().prev;
            hm.get_mut(left).unwrap().next = i;
            hm.get_mut(right).unwrap().prev = i;
            *hm.get_mut(i).unwrap() = Node {
                prev: left,
                next: right,
            };
        }
    }
    let mut ans = 0;
    let mut current = input
        .iter()
        .enumerate()
        .find_map(|(i, x)| (*x == 0).then_some(i))
        .unwrap();
    for _ in 0..1000 % input.len() {
        current = hm.get(current).unwrap().next;
    }
    ans += input[current];
    for _ in 0..1000 % input.len() {
        current = hm.get(current).unwrap().next;
    }
    ans += input[current];
    for _ in 0..1000 % input.len() {
        current = hm.get(current).unwrap().next;
    }
    ans += input[current];

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"1
2
-3
3
-2
0
4"#
    }

    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 3);
    }
}

