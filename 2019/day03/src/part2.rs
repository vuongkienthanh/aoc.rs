use crate::parsing::parse_input;
use aoc_helper::direction::many_steps;
use std::collections::BTreeMap;


pub fn process(_input: &str) -> usize {
    let (a, b) = parse_input(_input);
    let mut seen_a: BTreeMap<(isize, isize), usize> = BTreeMap::new();
    let (mut point, mut step) = ((0isize, 0isize), 0);
    for (dir, count) in a {
        for (x, y) in many_steps(dir, count) {
            step += 1;
            point = (point.0 + x, point.1 + y);
            seen_a
                .entry(point)
                .and_modify(|s| *s = (*s).min(step))
                .or_insert(step);
        }
    }
    let mut ans = usize::MAX;
    let (mut point, mut step) = ((0isize, 0isize), 0);
    for (dir, count) in b {
        for (x, y) in many_steps(dir, count) {
            step += 1;
            point = (point.0 + x, point.1 + y);
            if let Some(a_step) = seen_a.get(&point) {
                ans = ans.min(*a_step + step);
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"R8,U5,L5,D3
U7,R6,D4,L4"#,
        30
    )]
    #[case(
        r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#,
        610
    )]
    #[case(
        r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#,
        410
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
