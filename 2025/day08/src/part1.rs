use crate::parsing::{Point, parse_input};
use crate::to_sorted_pairs;
use std::collections::HashMap;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    part1(input, 1000)
}

fn part1(points: Vec<Point>, take: usize) -> usize {
    let pairs = to_sorted_pairs(&points);

    // index as circuits index, each item contains the points in same circuit
    let mut circuits: Vec<Vec<usize>> = vec![];

    // from each point to which circuit
    let mut mapping: HashMap<usize, usize> = HashMap::new();

    for (a, b) in pairs.into_iter().take(take) {
        match (mapping.get(&a).cloned(), mapping.get(&b).cloned()) {
            (Some(c1), Some(c2)) if c1 != c2 => {
                while let Some(p) = circuits[c2].pop() {
                    *mapping.get_mut(&p).unwrap() = c1;
                    circuits[c1].push(p);
                }
            }
            (Some(c1), None) => {
                circuits[c1].push(b);
                mapping.insert(b, c1);
            }
            (None, Some(c2)) => {
                circuits[c2].push(a);
                mapping.insert(a, c2);
            }
            (None, None) => {
                let c = circuits.len();
                circuits.push(vec![a, b]);
                mapping.insert(a, c);
                mapping.insert(b, c);
            }
            _ => (),
        }
    }
    let mut len = circuits.into_iter().map(|x| x.len()).collect::<Vec<_>>();
    len.sort_unstable();
    len.into_iter().rev().take(3).product()
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
        assert_eq!(part1(parse_input(fixture), 10), 40);
    }
}
