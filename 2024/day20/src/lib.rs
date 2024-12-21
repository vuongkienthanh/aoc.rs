pub mod part1;
pub mod part2;

use grid::Grid;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum CellType {
    Empty,
    Wall,
}

type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
struct CoordKey(usize, usize);

impl From<Coord> for CoordKey {
    fn from(value: Coord) -> Self {
        CoordKey(value.0, value.1)
    }
}

fn parse(input: &str) -> (Coord, Coord, Grid<CellType>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.char_indices() {
            match c {
                '#' => row.push(CellType::Wall),
                '.' => row.push(CellType::Empty),
                'S' => {
                    start = (i, j);
                    row.push(CellType::Empty);
                }
                'E' => {
                    end = (i, j);
                    row.push(CellType::Empty);
                }
                _ => (),
            }
        }
        grid.push(row);
    }
    let grid = Grid::from(grid);
    (start, end, grid)
}

fn mapping(start: Coord, end: Coord, grid: &Grid<CellType>) -> HashMap<CoordKey, usize> {
    let mut hm = HashMap::new();
    let mut runner = Some((end, start));
    let mut maxtime = 0;

    while let Some((prev_coord, this_coord)) = runner.take() {
        for adj in adjx(this_coord, 1, grid)
            .into_iter()
            .filter(|x| *x != prev_coord)
            .filter(|x| grid[*x] != CellType::Wall)
        {
            hm.insert(this_coord.into(), maxtime);
            maxtime += 1;
            runner.replace((this_coord, adj));
        }
    }

    hm.insert(end.into(), maxtime);
    hm
}

#[rustfmt::skip]
fn adjx(c: Coord, x: usize, grid: &Grid<CellType>) -> Vec<Coord> {
    let mut ans = vec![];
    if c.0 >= x { ans.push((c.0 - x, c.1)); }
    if c.0 + x < grid.rows() { ans.push((c.0 + x, c.1)); }
    if c.1 >= x { ans.push((c.0, c.1 - x)); }
    if c.1 + x < grid.cols() { ans.push((c.0, c.1 + x)); }
    for i in 1..x {
        if c.0 >= i {
            let rev = x-i;
            if c.1 >= rev { ans.push((c.0 - i, c.1 - rev)); }
            if c.1 + rev < grid.cols() { ans.push((c.0 - i, c.1 + rev)); }
        }
        if c.0 + i < grid.rows() {
            let rev = x-i;
            if c.1 >= rev { ans.push((c.0 + i, c.1 - rev)); }
            if c.1 + rev < grid.cols() { ans.push((c.0 + i, c.1 + rev)); }
        }
    }
    ans
}
