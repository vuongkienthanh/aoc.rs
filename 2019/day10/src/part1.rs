use crate::parsing::parse;
use fraction::GenericFraction;
use std::collections::BTreeSet;
type F = GenericFraction<usize>;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    find_best_location(&input).0
}

pub fn find_best_location(input: &[(usize, usize)]) -> (usize, (usize, usize)) {
    let mut ans = (0, (0, 0));
    for (x0, y0) in input {
        let mut count = BTreeSet::new();
        for (x1, y1) in input {
            if (x0, y0) == (x1, y1) {
                continue;
            }
            let (dx, dy) = (x0.abs_diff(*x1), y0.abs_diff(*y1));
            count.insert((F::new(dx, dy), x0 >= x1, y0 >= y1));
        }
        ans = ans.max((count.len(), (*x0, *y0)));
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#".#..#
.....
#####
....#
...##"#,
        8
    )]
    #[case(
        r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#,
        33
    )]
    #[case(
        r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#,
        35
    )]
    #[case(
        r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#,
        41
    )]
    #[case(
        r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#,
        210
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
