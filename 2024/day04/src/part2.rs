use super::parse;

pub fn process(_input: &str) -> usize {
    let grid = parse(_input);

    (1..grid.rows() - 1)
        .flat_map(|i| (1..grid.cols() - 1).map(move |j| (i, j)))
        .filter(|(row, col)| grid.get(*row, *col).unwrap() == &'A')
        .filter(|(row, col)| {
            [
                [
                    (row - 1, col - 1),
                    (row - 1, col + 1),
                    (row + 1, col + 1),
                    (row + 1, col - 1),
                ],
                [
                    (row - 1, col + 1),
                    (row + 1, col + 1),
                    (row + 1, col - 1),
                    (row - 1, col - 1),
                ],
                [
                    (row + 1, col + 1),
                    (row + 1, col - 1),
                    (row - 1, col - 1),
                    (row - 1, col + 1),
                ],
                [
                    (row + 1, col - 1),
                    (row - 1, col - 1),
                    (row - 1, col + 1),
                    (row + 1, col + 1),
                ],
            ]
            .into_iter()
            .any(|v| {
                v.map(|(row, col)| grid.get(row, col).unwrap())
                    .into_iter()
                    .zip("MSSM".chars())
                    .all(|(a, b)| a == &b)
            })
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(process(input), 9);
    }
}
