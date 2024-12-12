pub mod part1;
pub mod part2;

use grid::Grid;
fn parse(input: &str) -> Grid<char> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>()
        .into()
}

type Coord = (usize, usize);

#[derive(Eq, PartialEq, Hash)]
struct CoordKey {
    i: usize,
    j: usize,
}

fn adj4(i: usize, j: usize, rows: usize, cols: usize) -> Vec<Coord> {
    let mut ans = vec![];
    if i > 0 {
        ans.push((i - 1, j))
    };
    if i < rows - 1 {
        ans.push((i + 1, j))
    };
    if j > 0 {
        ans.push((i, j - 1))
    };
    if j < cols - 1 {
        ans.push((i, j + 1))
    };
    ans
}
