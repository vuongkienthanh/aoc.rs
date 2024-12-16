pub mod part1;
pub mod part2;

use grid::Grid;
use std::{
    collections::{HashMap, VecDeque},
    iter,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct CoordKey(usize, usize);
impl From<Coord> for CoordKey {
    fn from(value: Coord) -> Self {
        CoordKey(value.0, value.1)
    }
}

type Coord = (usize, usize);
#[derive(Debug, Default)]
struct Intersection {
    up: Option<(Coord, Direction, usize)>,
    down: Option<(Coord, Direction, usize)>,
    left: Option<(Coord, Direction, usize)>,
    right: Option<(Coord, Direction, usize)>,
}
impl Intersection {
    fn get_dir(&mut self, dir: Direction) -> &mut Option<(Coord, Direction, usize)> {
        match dir {
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> (Grid<char>, Coord, Coord, HashMap<CoordKey, Intersection>) {
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.char_indices() {
            match c {
                '#' | '.' => row.push(c),
                'S' => {
                    start = (i, j);
                    row.push('.');
                }
                'E' => {
                    end = (i, j);
                    row.push('.');
                }
                _ => (),
            }
        }
        grid.push(row);
    }
    let grid = Grid::from(grid);
    let mut nodes = vec![start, end];
    for ((i, j), _) in grid.indexed_iter().filter(|((i, j), c)| {
        **c == '.' && *i > 0 && *i < grid.rows() - 1 && *j > 0 && *j < grid.cols() - 1
    }) {
        if [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
            .into_iter()
            .filter(|x| grid[*x] == '.')
            .count()
            >= 3
        {
            nodes.push((i, j));
        }
    }

    let mut graph = HashMap::<CoordKey, Intersection>::new();
    for (x, y) in &nodes {
        let mut stack = VecDeque::from_iter(
            [
                ((*x - 1, *y), 1, Direction::Up, Direction::Up),
                ((*x + 1, *y), 1, Direction::Down, Direction::Down),
                ((*x, *y - 1), 1, Direction::Left, Direction::Left),
                ((*x, *y + 1), 1, Direction::Right, Direction::Right),
            ]
            .into_iter()
            .filter(|(node, _, _, _)| grid[*node] != '#'),
        );
        while let Some((n, cost, dir, origin_dir)) = stack.pop_front() {
            for (candidate, new_cost, new_dir) in step(n, cost, dir) {
                if grid[candidate] == '#' {
                    continue;
                } else if nodes.contains(&candidate) {
                    let itersection = graph
                        .entry(CoordKey::from((*x, *y)))
                        .or_default()
                        .get_dir(origin_dir);
                    if itersection.is_some_and(|old_cost| old_cost.2 > new_cost)
                        || itersection.is_none()
                    {
                        *itersection = Some((candidate, new_dir, new_cost));
                    }
                } else if grid[candidate] == '.' {
                    stack.push_back((candidate, new_cost, new_dir, origin_dir));
                }
            }
        }
    }

    (grid, start, end, graph)
}

fn step(c: Coord, count: usize, dir: Direction) -> [(Coord, usize, Direction); 3] {
    match dir {
        Direction::Up => [
            ((c.0 - 1, c.1), count + 1, dir),
            ((c.0, c.1 - 1), count + 1001, Direction::Left),
            ((c.0, c.1 + 1), count + 1001, Direction::Right),
        ],
        Direction::Down => [
            ((c.0 + 1, c.1), count + 1, dir),
            ((c.0, c.1 - 1), count + 1001, Direction::Left),
            ((c.0, c.1 + 1), count + 1001, Direction::Right),
        ],
        Direction::Left => [
            ((c.0, c.1 - 1), count + 1, dir),
            ((c.0 - 1, c.1), count + 1001, Direction::Up),
            ((c.0 + 1, c.1), count + 1001, Direction::Down),
        ],
        Direction::Right => [
            ((c.0, c.1 + 1), count + 1, dir),
            ((c.0 - 1, c.1), count + 1001, Direction::Up),
            ((c.0 + 1, c.1), count + 1001, Direction::Down),
        ],
    }
}
