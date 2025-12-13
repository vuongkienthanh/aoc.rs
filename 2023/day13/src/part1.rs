fn check_columns(block: &str) -> Option<usize> {
    let col_len = block.lines().next().unwrap().len();

    'outer: for col in 1..col_len {
        let left = block
            .lines()
            .map(|line| line.chars().nth(col - 1).unwrap())
            .collect::<Vec<char>>();
        let right = block
            .lines()
            .map(|line| line.chars().nth(col).unwrap())
            .collect::<Vec<char>>();

        if left == right {
            let number_of_left_cols = col - 1;
            let number_of_right_cols = col_len - col - 1;
            let compare_size = number_of_left_cols.min(number_of_right_cols);

            // todo!("if any not equal -> continue outer loop");
            for i in 0..compare_size {
                let temp_left = block
                    .lines()
                    .map(|line| line.chars().nth(col - 2 - i).unwrap())
                    .collect::<Vec<char>>();
                let temp_right = block
                    .lines()
                    .map(|line| line.chars().nth(col + 1 + i).unwrap())
                    .collect::<Vec<char>>();
                if temp_left != temp_right {
                    continue 'outer;
                }
            }

            // todo!("if all equal -> return col");
            return Some(col);
        }
    }
    return None;
}
fn check_rows(block: &str) -> Option<usize> {
    let row_len = block.lines().count();

    'outer: for row in 1..row_len {
        let above = block.lines().nth(row - 1).unwrap();
        let below = block.lines().nth(row).unwrap();

        if above == below {
            let number_of_above_rows = row - 1;
            let number_of_below_rows = row_len - row - 1;
            let compare_size = number_of_above_rows.min(number_of_below_rows);

            // todo!("if any not equal -> continue outer loop");
            for i in 0..compare_size {
                let temp_above = block.lines().nth(row - 2 - i).unwrap();
                let temp_below = block.lines().nth(row + 1 + i).unwrap();
                if temp_above != temp_below {
                    continue 'outer;
                }
            }

            // todo!("if all equal -> return col");
            return Some(row);
        }
    }
    return None;
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
        assert_eq!(process(input), 405);
    }
}
