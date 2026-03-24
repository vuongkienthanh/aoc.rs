pub mod part1;
pub mod part2;
pub mod parsing;

#[derive(Debug)]
pub enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Cell {
    Blizzard(Vec<(Blizzard, bool)>),
    Wall,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Blizzard(vec![])
    }
}

impl Cell {
    pub fn push(&mut self, b: Blizzard) {
        match self {
            Cell::Wall => panic!(),
            Cell::Blizzard(v) => v.push((b, true)),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Wall => false,
            Cell::Blizzard(v) => v.is_empty(),
        }
    }
}

pub fn iterate_map(map: &mut Vec<Vec<Cell>>) {
    for r in 1..map.len() - 1 {
        for c in 1..map[0].len() - 1 {
            let cell = std::mem::take(map.get_mut(r).unwrap().get_mut(c).unwrap());
            match cell {
                Cell::Wall => panic!(),
                Cell::Blizzard(v) => {
                    for (b, is_moved) in v {
                        if is_moved {
                            map.get_mut(r).unwrap().get_mut(c).unwrap().push(b);
                        } else {
                            match b {
                                Blizzard::Up => {
                                    let mut target_r = r - 1;
                                    if matches!(map[target_r][c], Cell::Wall) {
                                        target_r = map.len() - 2;
                                    }
                                    map.get_mut(target_r).unwrap().get_mut(c).unwrap().push(b);
                                }
                                Blizzard::Down => {
                                    let mut target_r = r + 1;
                                    if matches!(map[target_r][c], Cell::Wall) {
                                        target_r = 1;
                                    }
                                    map.get_mut(target_r).unwrap().get_mut(c).unwrap().push(b);
                                }
                                Blizzard::Left => {
                                    let mut target_c = c - 1;
                                    if matches!(map[r][target_c], Cell::Wall) {
                                        target_c = map[0].len() - 2;
                                    }
                                    map.get_mut(r).unwrap().get_mut(target_c).unwrap().push(b);
                                }
                                Blizzard::Right => {
                                    let mut target_c = c + 1;
                                    if matches!(map[r][target_c], Cell::Wall) {
                                        target_c = 1;
                                    }
                                    map.get_mut(r).unwrap().get_mut(target_c).unwrap().push(b);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn standby_map(map: &mut Vec<Vec<Cell>>) {
    let rows = map.len();
    let cols = map[0].len();
    for row in map[1..rows - 1].iter_mut() {
        for cell in row[1..cols - 1].iter_mut() {
            match cell {
                Cell::Wall => panic!(),
                Cell::Blizzard(v) => v.iter_mut().for_each(|(_, is_moved)| *is_moved = false),
            }
        }
    }
}