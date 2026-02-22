pub mod parsing;
pub mod part1;
pub mod part2;
use fxhash::{FxBuildHasher, FxHashMap};
use indexmap::IndexSet;
use parsing::Point;

fn normalize(mut input: Vec<Vec<Point>>) -> (IndexSet<Point, FxBuildHasher>, Vec<Point>) {
    let mut water: IndexSet<Point, FxBuildHasher> = input.pop().unwrap().into_iter().collect();
    let mut scanner_loc = vec![(0, 0, 0)];
    let mut current: Vec<(usize, [Vec<Point>; 24])> = input
        .into_iter()
        .map(|v| scanner_variations(&v))
        .enumerate()
        .collect();
    let mut last_check = 0;
    let mut diff_map: Vec<Vec<FxHashMap<Point, usize>>> =
        vec![vec![FxHashMap::default(); 24]; current.len()];

    while !current.is_empty() {
        let mut new = vec![];
        let new_last_check = water.len();
        for (i, scanner) in current {
            let mut found_diff = None;
            'w: for (a, b, c) in water.iter().skip(last_check) {
                for (j, v) in scanner.iter().enumerate() {
                    let map = diff_map.get_mut(i).unwrap().get_mut(j).unwrap();
                    for (x, y, z) in v {
                        let diff = (x - a, y - b, z - c);
                        let count = map.entry(diff).or_default();
                        *count += 1;
                        if *count == 12 {
                            found_diff = Some((diff, v));
                            break 'w;
                        }
                    }
                }
            }
            if let Some(((a, b, c), v)) = found_diff {
                scanner_loc.push((a, b, c));
                diff_map[i].clear();
                water.extend(v.into_iter().map(|(x, y, z)| (x - a, y - b, z - c)));
            } else {
                new.push((i, scanner));
            }
        }
        last_check = new_last_check;
        current = new;
    }
    (water, scanner_loc)
}

fn scanner_variations(scanner: &Vec<Point>) -> [Vec<Point>; 24] {
    [
        scanner.iter().map(|(a, b, c)| (*a, *b, *c)).collect(),
        scanner.iter().map(|(a, b, c)| (*a, *c, -*b)).collect(),
        scanner.iter().map(|(a, b, c)| (*a, -*b, -*c)).collect(),
        scanner.iter().map(|(a, b, c)| (*a, -*c, *b)).collect(),
        scanner.iter().map(|(a, b, c)| (-*a, *c, *b)).collect(),
        scanner.iter().map(|(a, b, c)| (-*a, *b, -*c)).collect(),
        scanner.iter().map(|(a, b, c)| (-*a, -*c, -*b)).collect(),
        scanner.iter().map(|(a, b, c)| (-*a, -*b, *c)).collect(),
        //
        scanner.iter().map(|(a, b, c)| (*b, *c, *a)).collect(),
        scanner.iter().map(|(a, b, c)| (*b, *a, -*c)).collect(),
        scanner.iter().map(|(a, b, c)| (*b, -*c, -*a)).collect(),
        scanner.iter().map(|(a, b, c)| (*b, -*a, *c)).collect(),
        scanner.iter().map(|(a, b, c)| (-*b, *a, *c)).collect(),
        scanner.iter().map(|(a, b, c)| (-*b, *c, -*a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*b, -*a, -*c)).collect(),
        scanner.iter().map(|(a, b, c)| (-*b, -*c, *a)).collect(),
        //
        scanner.iter().map(|(a, b, c)| (*c, *a, *b)).collect(),
        scanner.iter().map(|(a, b, c)| (*c, *b, -*a)).collect(),
        scanner.iter().map(|(a, b, c)| (*c, -*a, -*b)).collect(),
        scanner.iter().map(|(a, b, c)| (*c, -*b, *a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*c, *b, *a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*c, *a, -*b)).collect(),
        scanner.iter().map(|(a, b, c)| (-*c, -*b, -*a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*c, -*a, *b)).collect(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_scanner;
    #[test]
    fn test_scanner_variations() {
        let _input = r#"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7"#;
        let (_, scanner) = parse_scanner(_input).unwrap();
        let variations = scanner_variations(&scanner);
        for s in [
            r#"--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0"#,
            r#"--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8"#,
            r#"--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8"#,
            r#"--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8"#,
        ] {
            let (_, scanner) = parse_scanner(s).unwrap();
            println!("{scanner:?}");
            assert!(variations.contains(&scanner));
        }
    }
}
