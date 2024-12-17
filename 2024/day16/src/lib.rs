pub mod part1;
pub mod part2;
use grid::Grid;
use std::collections::{HashMap, VecDeque};
use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    fn get_dir(&self, dir: Direction) -> &Option<(Coord, Direction, usize)> {
        match dir {
            Direction::Up => &self.up,
            Direction::Down => &self.down,
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
    fn get_mut_dir(&mut self, dir: Direction) -> &mut Option<(Coord, Direction, usize)> {
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
impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
impl Direction {
    fn expand(&self) -> [(Direction, usize); 3] {
        match self {
            Direction::Up | Direction::Down => [
                (*self, 0),
                (Direction::Left, 1000),
                (Direction::Right, 1000),
            ],
            Direction::Left | Direction::Right => {
                [(*self, 0), (Direction::Up, 1000), (Direction::Down, 1000)]
            }
        }
    }
    fn move_coord(&self, c: Coord) -> Coord {
        match self {
            Direction::Up => (c.0 - 1, c.1),
            Direction::Down => (c.0 + 1, c.1),
            Direction::Left => (c.0, c.1 - 1),
            Direction::Right => (c.0, c.1 + 1),
        }
    }
}

fn parse_to_grid(input: &str) -> (Grid<char>, Coord, Coord) {
    let mut grid = vec![];
    let mut start = None;
    let mut end = None;
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.char_indices() {
            match c {
                '#' | '.' => row.push(c),
                'S' => {
                    start = Some((i, j));
                    row.push('.');
                }
                'E' => {
                    end = Some((i, j));
                    row.push('.');
                }
                _ => (),
            }
        }
        grid.push(row);
    }
    (Grid::from(grid), start.unwrap(), end.unwrap())
}

fn parse_to_graph(grid: &Grid<char>, start: Coord, end: Coord) -> HashMap<CoordKey, Intersection> {
    let mut graph = HashMap::<CoordKey, Intersection>::new();
    graph.insert(CoordKey::from(start), Intersection::default());
    graph.insert(CoordKey::from(end), Intersection::default());
    for ((i, j), _) in grid.indexed_iter().filter(|((i, j), c)| {
        **c == '.' && *i > 0 && *i < grid.rows() - 1 && *j > 0 && *j < grid.cols() - 1
    }) {
        if [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
            .into_iter()
            .filter(|x| grid[*x] == '.')
            .count()
            >= 3
        {
            graph.insert(CoordKey::from((i, j)), Intersection::default());
        }
    }

    for key in graph.keys().cloned().collect::<Vec<_>>() {
        let CoordKey(ndx, ndy) = key;
        let mut stack = VecDeque::from_iter(
            [
                ((ndx - 1, ndy), 1, Direction::Up, Direction::Up),
                ((ndx + 1, ndy), 1, Direction::Down, Direction::Down),
                ((ndx, ndy - 1), 1, Direction::Left, Direction::Left),
                ((ndx, ndy + 1), 1, Direction::Right, Direction::Right),
            ]
            .into_iter()
            .filter(|(node, _, _, _)| grid[*node] != '#')
            .filter(|(_, _, dir, _)| graph[&key].get_dir(*dir).is_none()),
        );
        while let Some((pos, cost, dir, origin_dir)) = stack.pop_front() {
            for (next_pos, next_cost, next_dir) in step(pos, cost, dir) {
                let next_pos_key = CoordKey::from(next_pos);
                if grid[next_pos] == '#' {
                    continue;
                } else if graph.contains_key(&next_pos_key) {
                    *graph.get_mut(&key).unwrap().get_mut_dir(origin_dir) =
                        Some((next_pos, next_dir, next_cost));
                    *graph.get_mut(&next_pos_key).unwrap().get_mut_dir(-next_dir) =
                        Some(((ndx, ndy), -origin_dir, next_cost));
                } else if grid[next_pos] == '.' {
                    stack.push_back((next_pos, next_cost, next_dir, origin_dir));
                }
            }
        }
    }

    graph
}

fn step(c: Coord, cost: usize, dir: Direction) -> [(Coord, usize, Direction); 3] {
    dir.expand().map(|(expand_dir, expand_cost)| {
        (expand_dir.move_coord(c), cost + expand_cost + 1, expand_dir)
    })
}

fn print_grid(grid: &Grid<char>, graph: &HashMap<CoordKey, Intersection>) {
    for (i, line) in grid.iter_rows().enumerate() {
        for (j, c) in line.enumerate() {
            if graph.contains_key(&CoordKey::from((i, j))) {
                print!("X");
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}
fn print_graph(graph: &HashMap<CoordKey, Intersection>) {
    for (k, v) in graph {
        println!(
            r#"
({}, {}):
    up:{:?}
    down:{:?}
    left:{:?}
    right:{:?}"#,
            k.0, k.1, v.up, v.down, v.left, v.right
        );
    }
}
