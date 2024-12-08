pub mod part1;
pub mod part2;

use grid::Grid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    West,
    East,
    South,
    North,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CellType {
    Empty,
    Obstacle,
}

pub type Coord = [usize; 2];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Guard {
    direction: Direction,
    position: Coord,
}
pub struct WalkResult {
    middle_path: Vec<Coord>,
    next_guard: Guard,
    is_stop: bool,
}
impl Guard {
    fn walk(&self, grid: &Grid<CellType>) -> WalkResult {
        match self.direction {
            Direction::North => {
                let mut middle_path = (0..self.position[0])
                    .rev()
                    .take_while(|i| *grid.get(*i, self.position[1]).unwrap() == CellType::Empty)
                    .map(|i| [i, self.position[1]])
                    .collect::<Vec<_>>();
                let next_guard = Guard {
                    direction: Direction::East,
                    position: middle_path.pop().unwrap_or(self.position),
                };
                let is_stop = next_guard.position[0] == 0;
                WalkResult {
                    middle_path,
                    next_guard,
                    is_stop,
                }
            }
            Direction::South => {
                let mut middle_path = (self.position[0] + 1..grid.rows())
                    .take_while(|i| *grid.get(*i, self.position[1]).unwrap() == CellType::Empty)
                    .map(|i| [i, self.position[1]])
                    .collect::<Vec<_>>();
                let next_guard = Guard {
                    direction: Direction::West,
                    position: middle_path.pop().unwrap_or(self.position),
                };
                let is_stop = next_guard.position[0] == grid.rows() - 1;
                WalkResult {
                    middle_path,
                    next_guard,
                    is_stop,
                }
            }
            Direction::West => {
                let mut middle_path = (0..self.position[1])
                    .rev()
                    .take_while(|j| *grid.get(self.position[0], *j).unwrap() == CellType::Empty)
                    .map(|j| [self.position[0], j])
                    .collect::<Vec<_>>();
                let next_guard = Guard {
                    direction: Direction::North,
                    position: middle_path.pop().unwrap_or(self.position),
                };
                let is_stop = next_guard.position[1] == 0;
                WalkResult {
                    middle_path,
                    next_guard,
                    is_stop,
                }
            }
            Direction::East => {
                let mut middle_path = (self.position[1] + 1..grid.cols())
                    .take_while(|j| *grid.get(self.position[0], *j).unwrap() == CellType::Empty)
                    .map(|j| [self.position[0], j])
                    .collect::<Vec<_>>();
                let next_guard = Guard {
                    direction: Direction::South,
                    position: middle_path.pop().unwrap_or(self.position),
                };
                let is_stop = next_guard.position[1] == grid.cols() - 1;
                WalkResult {
                    middle_path,
                    next_guard,
                    is_stop,
                }
            }
        }
    }
    fn jump(&self, grid: &Grid<CellType>) -> Option<Guard> {
        match self.direction {
            Direction::North => (0..self.position[0])
                .rev()
                .find(|i| *grid.get(*i, self.position[1]).unwrap() == CellType::Obstacle)
                .map(|i| Guard {
                    direction: Direction::East,
                    position: [i + 1, self.position[1]],
                }),
            Direction::South => (self.position[0] + 1..grid.rows())
                .find(|i| *grid.get(*i, self.position[1]).unwrap() == CellType::Obstacle)
                .map(|i| Guard {
                    direction: Direction::West,
                    position: [i - 1, self.position[1]],
                }),
            Direction::West => (0..self.position[1])
                .rev()
                .find(|j| *grid.get(self.position[0], *j).unwrap() == CellType::Obstacle)
                .map(|j| Guard {
                    direction: Direction::North,
                    position: [self.position[0], j + 1],
                }),
            Direction::East => (self.position[1] + 1..grid.cols())
                .find(|j| *grid.get(self.position[0], *j).unwrap() == CellType::Obstacle)
                .map(|j| Guard {
                    direction: Direction::South,
                    position: [self.position[0], j - 1],
                }),
        }
    }
}
pub fn parse(input: &str) -> (Guard, Grid<CellType>) {
    let mut guard = Guard {
        direction: Direction::North,
        position: [0, 0],
    };
    let mut grid = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut current_line = Vec::new();
        for (j, c) in line.char_indices() {
            match c {
                '^' => {
                    guard.position = [i, j];
                    current_line.push(CellType::Empty)
                }
                '#' => current_line.push(CellType::Obstacle),
                _ => current_line.push(CellType::Empty),
            }
        }
        grid.push(current_line);
    }
    (guard, Grid::from(grid))
}
