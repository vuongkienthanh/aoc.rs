pub mod parsing;
pub mod part1;
pub mod part2;
use parsing::Point;

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
        scanner.iter().map(|(a, b, c)| (*b, *a, *c)).collect(),
        scanner.iter().map(|(a, b, c)| (*b, *c, -*a)).collect(),
        scanner.iter().map(|(a, b, c)| (*b, -*a, -*c)).collect(),
        scanner.iter().map(|(a, b, c)| (*b, -*c, *a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*b, *c, *a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*b, *a, -*c)).collect(),
        scanner.iter().map(|(a, b, c)| (-*b, -*c, -*a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*b, -*a, *c)).collect(),
        //
        scanner.iter().map(|(a, b, c)| (*c, *a, *b)).collect(),
        scanner.iter().map(|(a, b, c)| (*c, *b, -*a)).collect(),
        scanner.iter().map(|(a, b, c)| (*c, -*a, -*b)).collect(),
        scanner.iter().map(|(a, b, c)| (*c, -*b, *a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*c, *b, *a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*c, *a, -*b)).collect(),
        scanner.iter().map(|(a, b, c)| (-*c, -*c, -*a)).collect(),
        scanner.iter().map(|(a, b, c)| (-*c, -*a, *c)).collect(),
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
            let (_, scanner) = parse_scanner(_input).unwrap();
            assert!(variations.contains(&scanner));
        }
    }
}
