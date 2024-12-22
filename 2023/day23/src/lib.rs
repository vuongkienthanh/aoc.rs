pub mod part1;
pub mod part2;
type Grid = Vec<Vec<char>>;
type Point = (usize, usize);
fn getxy(grid: &Grid, x: usize, y: usize) -> char {
    *grid.get(x).unwrap().get(y).unwrap()
}
