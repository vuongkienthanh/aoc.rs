pub mod part1;
pub mod part2;

use grid::Grid;
type Coord = [usize; 2];

fn directions<T>(coord: Coord, grid: &Grid<T>) -> Vec<Coord> {
    let mut ans = vec![];
    if coord[0] > 0 {
        ans.push([coord[0] - 1, coord[1]])
    };
    if coord[0] < grid.rows() - 1 {
        ans.push([coord[0] + 1, coord[1]])
    };
    if coord[1] > 0 {
        ans.push([coord[0], coord[1] - 1])
    };
    if coord[1] < grid.cols() - 1 {
        ans.push([coord[0], coord[1] + 1])
    };

    ans
}

fn parse(input: &str) -> Grid<u32> {
    Grid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}
