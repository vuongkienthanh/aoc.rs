use crate::{Beam, Direction, Puzzle};
use std::iter;

pub fn process(_input: &str) -> usize {
    let mut puzzle = Puzzle::new(_input);
    let max_rows = puzzle.max_rows;
    let max_cols = puzzle.max_cols;

    let row_indices = iter::repeat(0)
        .take(max_cols)
        .chain(0..max_rows)
        .chain(iter::repeat(max_rows - 1).take(max_cols))
        .chain((0..max_rows).rev());
    let col_indices = (0..max_cols)
        .chain(iter::repeat(max_cols - 1).take(max_rows))
        .chain((0..max_cols).rev())
        .chain(iter::repeat(0).take(max_rows));
    let direction_to_test = iter::repeat(Direction::Down)
        .take(max_cols)
        .chain(iter::repeat(Direction::Left).take(max_rows))
        .chain(iter::repeat(Direction::Up).take(max_cols))
        .chain(iter::repeat(Direction::Right).take(max_rows));
    // iterator ((row,col), direction)
    row_indices
        .zip(col_indices)
        .zip(direction_to_test)
        .map(|(coord, direction)| Beam {
            loc: coord,
            direction,
        })
        .map(|beam| puzzle.run(beam))
        .max()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(process(input), 51);
    }
}
