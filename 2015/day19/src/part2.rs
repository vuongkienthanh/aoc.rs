use crate::parsing::parse_input;
use std::collections::BinaryHeap;

pub fn process(_input: &str) -> usize {
    let (map, molecules) = parse_input(_input);
    let cmap = reverse_map(map);
    find_min(cmap, molecules)
}

fn reverse_map<'a>(map: Vec<(&'a str, Vec<&'a str>)>) -> Vec<(Vec<&'a str>, &'a str)> {
    map.into_iter().map(|(k, v)| (v, k)).collect()
}

fn compress_molecules<'a>(s: Vec<&'a str>, from: &Vec<&'a str>, to: &'a str) -> Vec<Vec<&'a str>> {
    if s.len() < from.len() {
        return vec![];
    }
    let mut i = 0;
    let mut ans = vec![];

    while i <= s.len() - from.len() {
        if s[i..].iter().zip(from).all(|(x, y)| x == y) {
            let mut new_s = s[..i].to_vec();
            new_s.push(to);
            new_s.extend(&s[i + from.len()..]);
            ans.push(new_s);
        }
        i += 1;
    }
    ans
}

fn find_min(cmap: Vec<(Vec<&str>, &str)>, molecules: Vec<&str>) -> usize {
    let mut heap = BinaryHeap::from([(0, molecules)]);

    while let Some((steps, molecules)) = heap.pop() {
        if molecules.len() == 1 && molecules[0] == "e" {
            return steps;
        }
        for (from, to) in &cmap {
            for new_molecules in compress_molecules(molecules.clone(), from, to) {
                heap.push((steps + 1, new_molecules));
            }
        }
    }
    panic!("should have an answer")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::{parse_map, parse_molecules};
    use rstest::*;
    #[fixture]
    fn fixture() -> Vec<(Vec<&'static str>, &'static str)> {
        let (_, map) = parse_map(
            r#"e => H
e => O
H => HO
H => OH
O => HH"#,
        )
        .unwrap();
        reverse_map(map)
    }

    #[rstest]
    #[case("HOH", 3)]
    #[case("HOHOHO", 6)]
    fn test_find_min(fixture: Vec<(Vec<&str>, &str)>, #[case] m: &str, #[case] expected: usize) {
        let (_, molecules) = parse_molecules(m).unwrap();
        assert_eq!(find_min(fixture, molecules), expected);
    }

    #[rstest]
    #[case(vec!["Ab", "Ab","Ti"], vec!["Ab", "Ab"], "B", vec![vec!["B", "Ti"]])]
    #[case(vec!["Ab", "Ab","Ti"], vec!["Ab", "Ti"], "B", vec![vec!["Ab", "B"]])]
    #[case(vec!["Ab", "Ab","Ti","Ab", "Ab"], vec!["Ab", "Ab"], "B", vec![vec!["B", "Ti","Ab", "Ab"], vec!["Ab", "Ab", "Ti", "B"]])]
    #[case(vec!["Ab", "Ab","Ti"], vec!["Ab", "E"], "B", vec![])]
    #[case(vec!["H"], vec!["H"], "e", vec![vec!["e"]])]
    fn test_compress_molecules(
        #[case] s: Vec<&str>,
        #[case] from: Vec<&str>,
        #[case] to: &str,
        #[case] x: Vec<Vec<&str>>,
    ) {
        let a = compress_molecules(s, &from, to);
        assert_eq!(a, x);
    }
}
