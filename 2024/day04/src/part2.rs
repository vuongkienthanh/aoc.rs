use grid::Grid;

pub fn process(_input: &str) -> usize {
    let grid = Grid::from(
        _input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    (1..grid.rows() - 1)
        .flat_map(|i| (1..grid.cols() - 1).map(move |j| (i, j)))
        .filter(|(row, col)| grid.get(*row, *col).unwrap() == &'A')
        .flat_map(|(row, col)| {
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
        })
        .map(|v| v.map(|(row, col)| grid.get(row, col).unwrap()))
        .filter(|v| v == &[&'M', &'S', &'S', &'M'])
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
