use crate::normalize;
use crate::parsing::parse_input;

type Point = (usize, usize);

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let input = normalize(input);
    count(&input, 10_000)
}

fn count(input: &[Point], max: usize) -> usize {
    let (max_row, max_col) = input.iter().fold((0, 0), |(max_row, max_col), (a, b)| {
        (max_row.max(*a), max_col.max(*b))
    });
    let (rows, cols) = (max_row + 1, max_col + 1);
    let mut ans = 0;
    for i in 0..rows {
        for j in 0..cols {
            if manhatan_distance((i, j), input) < max {
                ans += 1;
            }
        }
    }

    ans
}

fn manhatan_distance(p: Point, input: &[Point]) -> usize {
    input
        .iter()
        .map(|x| x.0.abs_diff(p.0) + x.1.abs_diff(p.1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        let input = parse_input(fixture);
        let input = normalize(input);
        assert_eq!(count(&input, 32), 16);
    }
}
