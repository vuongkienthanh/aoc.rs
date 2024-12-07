use super::parse;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let (mut guard, grid) = parse(_input);
    let mut visited = Grid::new(grid.rows(), grid.cols());
    visited.fill(0);
    visited[guard.position.into()] = 1;
    loop {
        let (this_time_visited, opt_next_guard) = guard.forward(&grid);
        for pos in this_time_visited {
            visited[pos.into()] = 1;
        }
        if let Some(next_guard) = opt_next_guard {
            guard = next_guard;
        } else {
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
