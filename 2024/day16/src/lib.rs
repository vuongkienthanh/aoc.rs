pub mod part1;
pub mod part2;
use grid::Grid;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CoordKey(usize, usize);

impl From<Coord> for CoordKey {
    fn from(value: Coord) -> Self {
        CoordKey(value.0, value.1)
    }
}

type Coord = (usize, usize);

/// path_to_dst exclude dst
#[derive(Debug)]
struct IntersectionDirInfo {
    dst: Coord,
    cost: usize,
    dir_at_dst: Direction,
    path_to_dst: Vec<Coord>,
}

#[derive(Debug, Default)]
struct Intersection {
    up: Option<IntersectionDirInfo>,
    down: Option<IntersectionDirInfo>,
    left: Option<IntersectionDirInfo>,
    right: Option<IntersectionDirInfo>,
}
impl Intersection {
    fn get_dir(&self, dir: Direction) -> &Option<IntersectionDirInfo> {
        match dir {
            Direction::Up => &self.up,
            Direction::Down => &self.down,
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
    fn get_mut_dir(&mut self, dir: Direction) -> &mut Option<IntersectionDirInfo> {
        match dir {
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Debug, Eq, PartialEq)]
enum CellType {
    Wall,
    Empty,
    Intersection,
}

fn parse(
    input: &str,
) -> (
    Coord,
    Coord,
    Grid<CellType>,
    HashMap<CoordKey, Intersection>,
) {
    let mut grid = vec![];
    let mut start = None;
    let mut end = None;
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.char_indices() {
            match c {
                '#' => row.push(CellType::Wall),
                '.' => row.push(CellType::Empty),
                'S' => {
                    start = Some((i, j));
                    row.push(CellType::Intersection);
                }
                'E' => {
                    end = Some((i, j));
                    row.push(CellType::Intersection);
                }
                _ => (),
            }
        }
        grid.push(row);
    }
    let mut grid = Grid::from(grid);
    let start = start.unwrap();
    let end = end.unwrap();

    let mut graph = HashMap::<CoordKey, Intersection>::new();
    graph.insert(start.into(), Intersection::default());
    graph.insert(end.into(), Intersection::default());

    for i in 1..grid.rows() - 1 {
        for j in 1..grid.cols() - 1 {
            if grid[(i, j)] == CellType::Empty
                && [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                    .into_iter()
                    .filter(|x| grid[*x] == CellType::Empty)
                    .count()
                    >= 3
            {
                grid[(i, j)] = CellType::Intersection;
                graph.insert((i, j).into(), Intersection::default());
            }
        }
    }
    for (coordkey, intersection) in graph.iter_mut() {
        let coord = (coordkey.0, coordkey.1);

        for origin_dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let next_coord = origin_dir.move_coord(coord);
            if grid[next_coord] != CellType::Wall {
                let visited = vec![next_coord];
                let mut stack = VecDeque::from([(next_coord, 1, origin_dir, visited)]);
                while let Some((coord, cost, dir, visited)) = stack.pop_front() {
                    for (next_coord, next_cost, next_dir) in step(coord, cost, dir) {
                        match grid[next_coord] {
                            CellType::Wall => continue,
                            CellType::Empty => {
                                let mut next_visited = visited.clone();
                                next_visited.push(next_coord);
                                stack.push_back((next_coord, next_cost, next_dir, next_visited));
                            }
                            CellType::Intersection => {
                                *intersection.get_mut_dir(origin_dir) = Some(IntersectionDirInfo {
                                    dst: next_coord,
                                    cost: next_cost,
                                    dir_at_dst: next_dir,
                                    path_to_dst: visited.clone(),
                                })
                            }
                        }
                    }
                }
            }
        }
    }
    if cfg!(test) {
        print_grid(&grid);
        print_graph(&graph);
    }
    (start, end, grid, graph)
}

fn step(c: Coord, cost: usize, dir: Direction) -> [(Coord, usize, Direction); 3] {
    dir.expand().map(|(expand_dir, expand_cost)| {
        (expand_dir.move_coord(c), cost + expand_cost + 1, expand_dir)
    })
}

fn print_grid(grid: &Grid<CellType>) {
    for line in grid.iter_rows() {
        for cell in line {
            match cell {
                CellType::Wall => print!("#"),
                CellType::Empty => print!("."),
                CellType::Intersection => print!("X"),
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
