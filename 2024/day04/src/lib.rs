pub mod part1;
pub mod part2;
use grid::Grid;

pub fn parse(input: &str) -> Grid<char> {
    Grid::from(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}
