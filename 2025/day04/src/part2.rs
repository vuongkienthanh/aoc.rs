use aoc_helper::adj::grid::adj8;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let mut grid: Grid<char> = _input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();

    let mut removed = 0;
    loop {
        let mut this_time_removed = 0;
        for i in 0..grid.rows() {
            for j in 0..grid.cols() {
                if grid[(i, j)] == '.' {
                    continue;
                }
                if adj8((i, j), grid.rows(), grid.cols())
                    .into_iter()
                    .flatten()
                    .filter(|p| grid[*p] == '@')
                    .count()
                    < 4
                {
                    grid[(i, j)] = '.';
                    this_time_removed += 1;
                }
            }
        }
        if this_time_removed == 0 {
            break;
        } else {
            removed += this_time_removed;
        }
    }

    removed
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 43);
    }
}
