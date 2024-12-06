use super::parse;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (mut guard, grid) = parse(_input);
    let mut visited = Grid::new(grid.rows(), grid.cols());
    visited.fill(0);
    visited[guard.position.into()] = 1;
    while let Some((_, next_guard)) = guard.try_forward(&grid) {
        guard = next_guard;
        visited[guard.position.into()] = 1;
    }
    visited.iter_rows().map(|row| row.sum::<usize>()).sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(process(input), 41);
    }
}
