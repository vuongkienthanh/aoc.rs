use super::{parse, WalkResult};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (mut guard, grid) = parse(_input);
    let mut visited = Grid::new(grid.rows(), grid.cols());
    visited[guard.position.into()] = 1;
    loop {
        let WalkResult {
            middle_path,
            next_guard,
            is_stop,
        } = guard.walk(&grid);

        // mark middle_path and next_guard as visited
        for pos in middle_path
            .into_iter()
            .chain(Some(&next_guard.position).cloned())
        {
            visited[pos.into()] = 1;
        }

        // prepare next loop
        if is_stop {
            break;
        }
        guard = next_guard;
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
