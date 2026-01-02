use crate::build_grid;
use aoc_helper::adj::naive::adj8;

pub fn process(_input: &str) -> String {
    let grid_serial_number: usize = _input.parse().unwrap();
    let (x, y) = find(grid_serial_number);
    format!("{x},{y}")
}

fn find(grid_serial_number: usize) -> (usize, usize) {
    let grid: [[isize; 300]; 300] = build_grid(grid_serial_number);

    (1..299)
        .flat_map(|y| (1..299).map(move |x| (x, y)))
        .max_by_key(|(x, y)| {
            grid[*y][*x]
                + adj8((*y, *x))
                    .into_iter()
                    .map(|(y, x)| grid[y][x])
                    .sum::<isize>()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(18, (33,45))]
    #[case(42, (21,61))]
    fn test_find(#[case] grid_serial_number: usize, #[case] expected: (usize, usize)) {
        assert_eq!(find(grid_serial_number), expected);
    }
}
