use super::parse;

pub fn process(_input: &str) -> usize {
    let grid = parse(_input);
    grid.indexed_iter()
        .filter(|(_, x)| **x == 'X')
        .flat_map(|((i, j), _)| {
            let mut res = vec![];

            if i >= 3 {
                res.push([1, 2, 3].map(|x| (i - x, j)));
                if j >= 3 {
                    res.push([1, 2, 3].map(|z| (i - z, j - z)))
                };
                if j + 3 < grid.cols() {
                    res.push([1, 2, 3].map(|z| (i - z, j + z)))
                };
            }
            if i + 3 < grid.rows() {
                res.push([1, 2, 3].map(|x| (i + x, j)));
                if j >= 3 {
                    res.push([1, 2, 3].map(|z| (i + z, j - z)))
                };
                if j + 3 < grid.cols() {
                    res.push([1, 2, 3].map(|z| (i + z, j + z)))
                };
            }
            if j >= 3 {
                res.push([1, 2, 3].map(|y| (i, j - y)))
            }
            if j + 3 < grid.cols() {
                res.push([1, 2, 3].map(|y| (i, j + y)))
            }

            res.into_iter()
        })
        .map(|v| v.map(|(row, col)| grid.get(row, col).unwrap()))
        .filter(|s| s == &[&'M', &'A', &'S'])
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
        assert_eq!(process(input), 18);
    }
}
