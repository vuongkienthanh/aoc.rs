pub mod parsing;
pub mod part1;
pub mod part2;

type Point = (usize, usize);

fn normalize(input: Vec<Point>) -> Vec<Point> {
    let (min_row, min_col) = input
        .iter()
        .fold((usize::MAX, usize::MAX), |(min_row, min_col), (a, b)| {
            (min_row.min(*a), min_col.min(*b))
        });
    input
        .into_iter()
        .map(|(a, b)| (a - min_row, b - min_col))
        .collect()
}

fn get_rows_cols(input: &[Point]) -> (usize, usize) {
    let (max_row, max_col) = input.iter().fold((0, 0), |(max_row, max_col), (a, b)| {
        (max_row.max(*a), max_col.max(*b))
    });
    (max_row + 1, max_col + 1)
}
