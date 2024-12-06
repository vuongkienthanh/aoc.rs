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

#[derive(Debug, Eq, PartialEq)]
pub enum MoveType {
    Forward,
    Turn {
        guard_direction_when_approach_obstacle: Direction,
        obstacle_position: Coord,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Guard {
    direction: Direction,
    position: Coord,
}
impl Guard {
    fn change_direction(mut self, direction: Direction) -> Guard {
        self.direction = direction;
        self
    }
    fn try_up(mut self) -> Option<Guard> {
        (self.position[0] > 0).then(|| {
            self.position[0] -= 1;
            self
        })
    }
    fn try_down(mut self, grid: &Grid<CellType>) -> Option<Guard> {
        (self.position[0] < grid.rows() - 1).then(|| {
            self.position[0] += 1;
            self
        })
    }
    fn try_left(mut self) -> Option<Guard> {
        (self.position[1] > 0).then(|| {
            self.position[1] -= 1;
            self
        })
    }
    fn try_right(mut self, grid: &Grid<CellType>) -> Option<Guard> {
        (self.position[1] < grid.cols() - 1).then(|| {
            self.position[1] += 1;
            self
        })
    }
    fn try_forward(&self, grid: &Grid<CellType>) -> Option<(MoveType, Guard)> {
        let (try_candidate_fw, candidate_turn) = match self.direction {
            Direction::West => (
                self.clone().try_left(),
                self.clone().change_direction(Direction::North),
            ),
            Direction::East => (
                self.clone().try_right(grid),
                self.clone().change_direction(Direction::South),
            ),
            Direction::South => (
                self.clone().try_down(grid),
                self.clone().change_direction(Direction::West),
            ),
            Direction::North => (
                self.clone().try_up(),
                self.clone().change_direction(Direction::East),
            ),
        };
        if let Some(candidate_fw) = try_candidate_fw {
            match grid[candidate_fw.position.into()] {
                CellType::Obstacle => Some((
                    MoveType::Turn {
                        guard_direction_when_approach_obstacle: candidate_fw.direction,
                        obstacle_position: candidate_fw.position,
                    },
                    candidate_turn,
                )),
                CellType::Empty => Some((MoveType::Forward, candidate_fw)),
            }
        } else {
            None
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
