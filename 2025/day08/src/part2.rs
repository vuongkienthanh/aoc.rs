use crate::parsing::parse_input;
use crate::to_sorted_pairs;
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let points = parse_input(_input);
    let pairs = to_sorted_pairs(&points);

    let mut seen: HashSet<usize> = HashSet::new();

    for (a, b) in pairs {
        seen.insert(a);
        seen.insert(b);
        if seen.len() == points.len() {
            return points[a].0 * points[b].0;
        }
    }
    panic!("should have an answer")
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#
    }
    #[rstest]
    fn test_process(fixture: &str) {
        assert_eq!(process(fixture), 25272);
    }
}
