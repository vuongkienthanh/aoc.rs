use grid::Grid;
use std::collections::HashSet;

fn to_grid(input: &str) -> ((isize, isize), Grid<char>) {
    let mut start = None;
    'outer: for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((x as isize, y as isize));
                break 'outer;
            }
        }
    }
    let grid = Grid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| if c == 'S' { '.' } else { c })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    (start.unwrap(), grid)
}

pub fn process(input: &str, steps: usize) -> usize {
    let (start, map) = to_grid(input);
    let mut set = HashSet::from([start]);
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn test_process_2a() {
        assert_eq!(process(INPUT, 6), 16);
    }
    #[test]
    fn test_process_2b() {
        assert_eq!(process(INPUT, 10), 50);
    }
    #[test]
    fn test_process_2c() {
        assert_eq!(process(INPUT, 50), 1594);
    }
    #[test]
    fn test_process_2d() {
        assert_eq!(process(INPUT, 100), 6536);
    }
    #[test]
    fn test_process_2e() {
        assert_eq!(process(INPUT, 500), 167004);
    }
    #[test]
    fn test_process_2f() {
        assert_eq!(process(INPUT, 1000), 668697);
    }
    #[test]
    fn test_process_2g() {
        assert_eq!(process(INPUT, 5000), 16733044);
    }
}
