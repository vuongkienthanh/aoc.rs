use grid::Grid;

pub fn parse_grid(input: &str) -> Grid<usize> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
        .into()
}
