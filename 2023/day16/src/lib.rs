pub mod part1;
pub mod part2;

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    loc: Coord,
    direction: Direction,
}
impl Beam {
    #[rustfmt::skip]
    fn right(&self) -> Self { Beam { loc: self.loc, direction: Direction::Right } }
    #[rustfmt::skip]
    fn left(&self) -> Self { Beam { loc: self.loc, direction: Direction::Left } }
    #[rustfmt::skip]
    fn up(&self) -> Self { Beam { loc: self.loc, direction: Direction::Up } }
    #[rustfmt::skip]
    fn down(&self) -> Self { Beam { loc: self.loc, direction: Direction::Down } }

    fn encounter(&self, point: char, max_rows: usize, max_cols: usize) -> Vec<Beam> {
        let mut collected = match point {
            '.' => vec![self.next_beam(max_rows, max_cols)],
            '|' => match self.direction {
                Direction::Left | Direction::Right => vec![
                    self.up().next_beam(max_rows, max_cols),
                    self.down().next_beam(max_rows, max_cols),
                ],
                _ => vec![self.next_beam(max_rows, max_cols)],
            },
            '-' => match self.direction {
                Direction::Up | Direction::Down => vec![
                    self.left().next_beam(max_rows, max_cols),
                    self.right().next_beam(max_rows, max_cols),
                ],
                _ => vec![self.next_beam(max_rows, max_cols)],
            },
            '/' => match self.direction {
                Direction::Up => vec![self.right().next_beam(max_rows, max_cols)],
                Direction::Down => vec![self.left().next_beam(max_rows, max_cols)],
                Direction::Left => vec![self.down().next_beam(max_rows, max_cols)],
                Direction::Right => vec![self.up().next_beam(max_rows, max_cols)],
            },
            '\\' => match self.direction {
                Direction::Up => vec![self.left().next_beam(max_rows, max_cols)],
                Direction::Down => vec![self.right().next_beam(max_rows, max_cols)],
                Direction::Left => vec![self.up().next_beam(max_rows, max_cols)],
                Direction::Right => vec![self.down().next_beam(max_rows, max_cols)],
            },
            _ => panic!("cant parse point"),
        };
        collected.into_iter().filter_map(|beam| beam).collect()
    }

    #[rustfmt::skip]
    fn next_beam(&self, max_rows:usize, max_cols:usize) -> Option<Self> {
        match self.direction {
            Direction::Up => {
                if self.loc.0 == 0 { None } else {
                    Some(Beam { loc: (self.loc.0 - 1, self.loc.1), direction: Direction::Up })
                }
            }
            Direction::Down => {
                if self.loc.0 + 1 == max_rows { None } else {
                    Some(Beam { loc: (self.loc.0 + 1, self.loc.1), direction: Direction::Down })
                }
            }
            Direction::Left => {
                if self.loc.1 == 0 { None } else {
                    Some(Beam { loc: (self.loc.0, self.loc.1 - 1), direction: Direction::Left })
                }
            }
            Direction::Right => {
                if self.loc.1 + 1 == max_cols { None } else {
                    Some(Beam { loc: (self.loc.0, self.loc.1 + 1), direction: Direction::Right })
                }
            }
        }
    }
}
