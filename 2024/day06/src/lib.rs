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
impl Guard {
    /// return visited coords, some next_guard if touch obs, else at edge return none
    fn forward(&self, grid: &Grid<CellType>) -> (Vec<Coord>, Option<Guard>) {
        match self.direction {
            Direction::North => {
                let visited = (0..self.position[0])
                    .rev()
                    .take_while(|i| *grid.get(*i, self.position[1]).unwrap() == CellType::Empty)
                    .map(|i| [i, self.position[1]])
                    .collect::<Vec<_>>();
                let next_guard = (visited.last().unwrap()[0] > 0).then_some(Guard {
                    direction: Direction::East,
                    position: *visited.last().unwrap(),
                });
                (visited, next_guard)
            }
            Direction::South => {
                let visited = (self.position[0] + 1..grid.rows())
                    .take_while(|i| *grid.get(*i, self.position[1]).unwrap() == CellType::Empty)
                    .map(|i| [i, self.position[1]])
                    .collect::<Vec<_>>();
                let next_guard = (visited.last().unwrap()[0] < grid.rows() - 1).then_some(Guard {
                    direction: Direction::West,
                    position: *visited.last().unwrap(),
                });
                (visited, next_guard)
            }
            Direction::West => {
                let visited = (0..self.position[1])
                    .rev()
                    .take_while(|j| *grid.get(self.position[0], *j).unwrap() == CellType::Empty)
                    .map(|j| [self.position[0], j])
                    .collect::<Vec<_>>();
                let next_guard = (visited.last().unwrap()[1] > 0).then_some(Guard {
                    direction: Direction::North,
                    position: *visited.last().unwrap(),
                });
                (visited, next_guard)
            }
            Direction::East => {
                let visited = (self.position[1] + 1..grid.cols())
                    .take_while(|j| *grid.get(self.position[0], *j).unwrap() == CellType::Empty)
                    .map(|j| [self.position[0], j])
                    .collect::<Vec<_>>();
                let next_guard = (visited.last().unwrap()[1] < grid.cols() - 1).then_some(Guard {
                    direction: Direction::South,
                    position: *visited.last().unwrap(),
                });
                (visited, next_guard)
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
