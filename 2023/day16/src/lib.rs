use std::collections::{HashMap, HashSet, VecDeque};

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
        match point {
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
        }
        .into_iter()
        .filter_map(|beam| beam)
        .collect()
    }

    #[rustfmt::skip]
    fn next_beam(&self, max_rows:usize, max_cols:usize) -> Option<Self> {
        match self.direction {
            Direction::Up => if self.loc.0 == 0 { None } else { Some(Beam { loc: (self.loc.0 - 1, self.loc.1), direction: Direction::Up }) }
            Direction::Down => if self.loc.0 + 1 == max_rows { None } else { Some(Beam { loc: (self.loc.0 + 1, self.loc.1), direction: Direction::Down }) }
            Direction::Left => if self.loc.1 == 0 { None } else { Some(Beam { loc: (self.loc.0, self.loc.1 - 1), direction: Direction::Left }) }
            Direction::Right => if self.loc.1 + 1 == max_cols { None } else { Some(Beam { loc: (self.loc.0, self.loc.1 + 1), direction: Direction::Right }) }
        }
    }
}

struct Puzzle<'a> {
    input: &'a str,
    cache: HashMap<Beam, (HashSet<Beam>, HashSet<Coord>)>,
    max_rows: usize,
    max_cols: usize,
}

impl<'a> Puzzle<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            cache: HashMap::new(),
            max_rows: input.lines().count(),
            max_cols: input.lines().next().unwrap().len(),
        }
    }

    #[rustfmt::skip]
    fn char_at(&self, loc: Coord) -> char {
        self.input.lines().nth(loc.0).unwrap().chars().nth(loc.1).unwrap()
    }

    // return set of beams at dst symbols, and energized tiles excluding src & dst
    fn forward_symbol_to_symbols(&mut self, src: Beam) -> (HashSet<Beam>, HashSet<Coord>) {
        if let Some(res) = self.cache.get(&src) {
            return res.clone();
        } else {
            let mut dst = HashSet::new();
            let mut tiles = HashSet::new();
            let mut beams =
                VecDeque::from(src.encounter(self.char_at(src.loc), self.max_rows, self.max_cols));
            while let Some(beam) = beams.pop_front() {
                if self.char_at(beam.loc) == '.' {
                    tiles.insert(beam.loc);
                    if let Some(nxt_beam) = beam.next_beam(self.max_rows, self.max_cols) {
                        beams.push_back(nxt_beam);
                    }
                } else {
                    dst.insert(beam);
                }
            }
            self.cache.insert(src, (dst.clone(), tiles.clone()));
            (dst, tiles)
        }
    }
    fn run(&mut self, start: Beam) -> usize {
        let mut energized = HashSet::new();
        let mut encounter_mirrors: HashSet<Beam> = HashSet::new();
        let mut beams = VecDeque::from([start]);

        while let Some(beam) = beams.pop_front() {
            energized.insert(beam.loc);
            let point = self.char_at(beam.loc);

            if point == '.' {
                let after_encounter = beam.encounter(point, self.max_rows, self.max_cols);
                for nxt_beam in after_encounter {
                    beams.push_back(nxt_beam);
                }
            } else {
                let (dst, tiles) = self.forward_symbol_to_symbols(beam);
                for nxt_beam in dst {
                    if !encounter_mirrors.contains(&nxt_beam) {
                        encounter_mirrors.insert(nxt_beam.clone());
                        beams.push_back(nxt_beam);
                    }
                }
                for t in tiles {
                    energized.insert(t);
                }
            }
        }

        energized.len()
    }
}
