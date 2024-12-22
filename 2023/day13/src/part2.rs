fn two_vec_char_diff(a: &[char], b: &[char]) -> usize {
    a.into_iter()
        .zip(b.into_iter())
        .filter(|(x, y)| x != y)
        .count()
}

fn check_columns(block: &str) -> Option<usize> {
    let col_len = block.lines().next().unwrap().len();
    for col in 1..col_len {
        let number_of_left_cols = col;
        let number_of_right_cols = col_len - col;
        let span = number_of_left_cols.min(number_of_right_cols);
        let diff: usize = (0..span)
            .map(|i| {
                let left = block
                    .lines()
                    .map(|line| line.chars().nth(col - 1 - i).unwrap())
                    .collect::<Vec<char>>();
                let right = block
                    .lines()
                    .map(|line| line.chars().nth(col + i).unwrap())
                    .collect::<Vec<char>>();
                two_vec_char_diff(&left, &right)
            })
            .sum();
        if diff == 1 {
            return Some(col);
        }
    }
    None
}

fn check_rows(block: &str) -> Option<usize> {
    let row_len = block.lines().count();
    for row in 1..row_len {
        let number_of_above_rows = row;
        let number_of_below_rows = row_len - row;
        let span = number_of_above_rows.min(number_of_below_rows);
        let diff: usize = (0..span)
            .map(|i| {
                let above = block
                    .lines()
                    .nth(row - 1 - i)
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                let below = block
                    .lines()
                    .nth(row + i)
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                two_vec_char_diff(&above, &below)
            })
            .sum();
        if diff == 1 {
            return Some(row);
        }
    }
    None
}
pub fn process(_input: &str) -> usize {
    _input
        .split("\n\n")
        .map(|block| match check_columns(block) {
            Some(cscore) => cscore,
            None => match check_rows(block) {
                Some(rscore) => rscore * 100,
                None => panic!("couldn't find mirror"),
            },
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(process(input), 400);
    }
}
