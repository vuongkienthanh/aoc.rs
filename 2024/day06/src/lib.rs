pub mod part1;
pub mod part2;

use grid::Grid;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

#[derive(Clone, Debug)]
pub struct Guard {
    direction: Direction,
    position: Coord,
}
pub struct GuardForwardResult {
    visited: Vec<Coord>,
    next_guard: Guard,
    is_stop: bool,
}
impl Guard {
    /// return visited coords, next_guard, stop
    fn forward(&self, grid: &Grid<CellType>) -> GuardForwardResult {
        match self.direction {
            Direction::North => {
                if let Some((i, _)) = (0..self.position[0])
                    .rev()
                    .map(|i| (i, grid.get(i, self.position[1]).unwrap()))
                    .find(|(_, ele)| **ele == CellType::Obstacle)
                {
                    let visited = (i + 1..self.position[0])
                        .map(|x| [x, self.position[1]])
                        .collect::<Vec<_>>();
                    let next_guard = Guard {
                        direction: Direction::East,
                        position: [i + 1, self.position[1]],
                    };
                    GuardForwardResult {
                        visited,
                        next_guard,
                        is_stop: false,
                    }
                } else {
                    let visited = (0..self.position[0])
                        .map(|x| [x, self.position[1]])
                        .collect::<Vec<_>>();
                    let next_guard = Guard {
                        direction: self.direction.clone(),
                        position: [0, self.position[1]],
                    };
                    GuardForwardResult {
                        visited,
                        next_guard,
                        is_stop: true,
                    }
                }
            }
            Direction::South => {
                if let Some((i, _)) = (self.position[0] + 1..grid.rows())
                    .map(|i| (i, grid.get(i, self.position[1]).unwrap()))
                    .find(|(_, ele)| **ele == CellType::Obstacle)
                {
                    let visited = (self.position[0] + 1..i)
                        .map(|x| [x, self.position[1]])
                        .collect::<Vec<_>>();
                    let next_guard = Guard {
                        direction: Direction::West,
                        position: [i - 1, self.position[1]],
                    };
                    GuardForwardResult {
                        visited,
                        next_guard,
                        is_stop: false,
                    }
                } else {
                    let visited = (self.position[0] + 1..grid.rows())
                        .map(|x| [x, self.position[1]])
                        .collect::<Vec<_>>();
                    let next_guard = Guard {
                        direction: self.direction.clone(),
                        position: [grid.rows() - 1, self.position[1]],
                    };
                    GuardForwardResult {
                        visited,
                        next_guard,
                        is_stop: true,
                    }
                }
            }
            Direction::East => {
                if let Some((j, _)) = (self.position[1] + 1..grid.cols())
                    .map(|j| (j, grid.get(self.position[0], j).unwrap()))
                    .find(|(_, ele)| **ele == CellType::Obstacle)
                {
                    let visited = (self.position[1] + 1..j)
                        .map(|y| [self.position[0], y])
                        .collect::<Vec<_>>();
                    let next_guard = Guard {
                        direction: Direction::South,
                        position: [self.position[0], j - 1],
                    };
                    GuardForwardResult {
                        visited,
                        next_guard,
                        is_stop: false,
                    }
                } else {
                    let visited = (self.position[1] + 1..grid.cols())
                        .map(|y| [self.position[0], y])
                        .collect::<Vec<_>>();
                    let next_guard = Guard {
                        direction: self.direction.clone(),
                        position: [self.position[0], grid.cols() - 1],
                    };
                    GuardForwardResult {
                        visited,
                        next_guard,
                        is_stop: true,
                    }
                }
            }
            Direction::West => {
                if let Some((j, _)) = (0..self.position[1])
                    .rev()
                    .map(|j| (j, grid.get(self.position[0], j).unwrap()))
                    .find(|(_, ele)| **ele == CellType::Obstacle)
                {
                    let visited = (j + 1..self.position[1])
                        .map(|y| [self.position[0], y])
                        .collect::<Vec<_>>();
                    let next_guard = Guard {
                        direction: Direction::North,
                        position: [self.position[0], j + 1],
                    };
                    GuardForwardResult {
                        visited,
                        next_guard,
                        is_stop: false,
                    }
                } else {
                    let visited = (0..self.position[1])
                        .map(|y| [self.position[0], y])
                        .collect::<Vec<_>>();
                    let next_guard = Guard {
                        direction: self.direction.clone(),
                        position: [self.position[0], 0],
                    };
                    GuardForwardResult {
                        visited,
                        next_guard,
                        is_stop: true,
                    }
                }
            }
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
