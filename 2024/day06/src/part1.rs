use super::{parse, GuardForwardResult};
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (mut guard, grid) = parse(_input);
    let mut visited = Grid::new(grid.rows(), grid.cols());
    visited.fill(0);
    visited[guard.position.into()] = 1;
    loop {
        let GuardForwardResult {
            visited: this_time_visited,
            next_guard,
            is_stop,
        } = guard.forward(&grid);
        guard = next_guard;
        for pos in this_time_visited {
            visited[pos.into()] = 1;
        }
        if is_stop {
            break;
        }
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
