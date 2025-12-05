use aoc_helper::adj::grid::adj8;
use grid::Grid;

pub fn process(_input: &str) -> usize {
    let mut grid: Grid<char> = _input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();

    let mut removed = 0;
    let mut rolls: Vec<(usize, usize)> = grid
        .indexed_iter()
        .filter(|(_, e)| e == &&'@')
        .map(|(i, _)| i)
        .collect();

    loop {
        let mut this_time_removed = 0;
        let mut new_rolls = vec![];
        for r in rolls {
            if adj8(r, grid.rows(), grid.cols())
                .into_iter()
                .flatten()
                .filter(|p| grid[*p] == '@')
                .count()
                < 4
            {
                grid[r] = '.';
                this_time_removed += 1;
            } else {
                new_rolls.push(r)
            }
        }

        if this_time_removed == 0 {
            break;
        } else {
            removed += this_time_removed;
            rolls = new_rolls;
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
