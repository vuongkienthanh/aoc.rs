#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Cell {
    A,
    B,
    C,
    D,
    E,
}
impl From<char> for Cell {
    fn from(value: char) -> Cell {
        match value {
            'A' => Cell::A,
            'B' => Cell::B,
            'C' => Cell::C,
            'D' => Cell::D,
            _ => panic!(),
        }
    }
}

fn amphipod_room(cell: &Cell) -> [usize; 2] {
    match cell {
        Cell::A => [0, 1],
        Cell::B => [2, 3],
        Cell::C => [4, 5],
        Cell::D => [6, 7],
        Cell::E => panic!(),
    }
}

fn get_movable_from(locations: &[Cell; 15]) -> Vec<usize> {
    let mut ans = vec![];
    let mut rs = vec![[9, 8], [13, 14]];
    if locations[0..2] != [Cell::A, Cell::A] || locations[0..2] != [Cell::A, Cell::E] {
        rs.push([1, 0]);
    }
    if locations[2..4] != [Cell::B, Cell::B] || locations[2..4] != [Cell::B, Cell::E] {
        rs.push([3, 2]);
    }
    if locations[4..6] != [Cell::C, Cell::C] || locations[4..6] != [Cell::C, Cell::E] {
        rs.push([5, 4]);
    }
    if locations[6..8] != [Cell::D, Cell::D] || locations[6..8] != [Cell::D, Cell::E] {
        rs.push([7, 6]);
    }
    for r in rs {
        if let Some(i) = r.into_iter().find(|x| !matches!(locations[*x], Cell::E)) {
            ans.push(i);
        }
    }
    ans.extend(
        [10, 11, 12]
            .into_iter()
            .filter(|x| !matches!(locations[*x], Cell::E)),
    );
    ans
}
// ####1#1#1#11#
// #89.0.1.2.34#
// ###1#3#5#7###
//   #0#2#4#6#
//   #########

fn get_movable_to(locations: &[Cell; 15], from: usize) -> Vec<usize> {
    let mut ans = vec![];
    match from {
        0 | 1 => {
            for r in [vec![8, 9], vec![14, 13, 12, 11, 10]] {
                if let Some(i) = r.into_iter().find(|x| matches!(locations[*x], Cell::E)) {
                    ans.push(i);
                }
            }
        }
        2 | 3 => {
            for r in [vec![8, 9, 10], vec![14, 13, 12, 11]] {
                if let Some(i) = r.into_iter().find(|x| matches!(locations[*x], Cell::E)) {
                    ans.push(i);
                }
            }
        }
        4 | 5 => {
            for r in [vec![8, 9, 10, 11], vec![15, 13, 12]] {
                if let Some(i) = r.into_iter().find(|x| matches!(locations[*x], Cell::E)) {
                    ans.push(i);
                }
            }
        }
        6 | 7 => {
            for r in [vec![8, 9, 10, 11, 12], vec![14, 13]] {
                if let Some(i) = r.into_iter().find(|x| matches!(locations[*x], Cell::E)) {
                    ans.push(i);
                }
            }
        }
        _ => {
            let amphipod = locations[from];
            let its_room = amphipod_room(&amphipod);
            if its_room
                .iter()
                .all(|x| locations[*x] == amphipod || locations[*x] == Cell::E)
            {
                let room = its_room
                    .into_iter()
                    .find(|x| matches!(locations[*x], Cell::E))
                    .unwrap();
                let obstacles = obstacle(from, room);
                if obstacles
                    .into_iter()
                    .all(|x| matches!(locations[x], Cell::E))
                {
                    ans.push(room);
                }
            }
        }
    }
    ans
}

#[derive(Debug)]
pub struct State {
    pub locations: [Cell; 15],
    pub last_moved: usize,
    pub score: usize,
}
impl From<&str> for State {
    fn from(value: &str) -> State {
        let input = value
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut locations = [Cell::E; 15];
        for (r, c, i) in [
            (3, 3, 0),
            (2, 3, 1),
            (3, 5, 2),
            (2, 5, 3),
            (3, 7, 4),
            (2, 7, 5),
            (3, 9, 6),
            (2, 9, 7),
        ] {
            locations[i] = input[r][c].into();
        }
        State {
            locations,
            last_moved: 15,
            score: 0,
        }
    }
}
impl State {
    pub fn is_done(&self) -> bool {
        self.locations[0..8]
            == [
                Cell::A,
                Cell::A,
                Cell::B,
                Cell::B,
                Cell::C,
                Cell::C,
                Cell::D,
                Cell::D,
            ]
    }
}

fn step(from: usize, to: usize) -> usize {
    match (from, to) {
        (0, 9) | (0, 10) | (2, 10) | (2, 11) | (4, 11) | (4, 12) | (6, 12) | (6, 13) => 3,
        (0, 11) | (2, 9) | (2, 12) | (4, 10) | (4, 13) | (6, 11) => 5,
        (0, 12) | (2, 13) | (4, 9) | (6, 10) => 7,
        (0, 13) | (6, 9) => 9,
        (x, 8) => step(x, 9) + 1,
        (x, 14) => step(x, 13) + 1,
        (1, y) => step(0, y) - 1,
        (3, y) => step(2, y) - 1,
        (5, y) => step(4, y) - 1,
        (7, y) => step(6, y) - 1,
        (x, y) if x >= 8 && y >= 8 => panic!("hall to hall"),
        (x, y) if x < 8 && y < 8 => panic!("room to room"),
        (x, y) => step(y, x),
    }
}
fn obstacle(from: usize, to: usize) -> Vec<usize> {
    match (from, to) {
        (9, 0) | (10, 0) | (10, 2) | (11, 2) | (11, 4) | (12, 4) | (12, 6) | (13, 6) => vec![],
        (9, 2) | (11, 0) => vec![10],
        (9, 4) | (12, 0) => vec![10, 11],
        (9, 6) | (13, 0) => vec![10, 11, 12],
        (10, 4) | (12, 2) => vec![11],
        (10, 6) | (13, 2) => vec![11, 12],
        (11, 6) | (13, 4) => vec![12],
        (8, y) => obstacle(9, y),
        (14, y) => obstacle(13, y),
        (x, 1) => obstacle(x, 0),
        (x, 3) => obstacle(x, 2),
        (x, 5) => obstacle(x, 4),
        (x, 7) => obstacle(x, 6),
        (x, y) if x >= 8 && y >= 8 => panic!("hall to hall"),
        (x, y) if x < 8 && y < 8 => panic!("room to room"),
        (x, y) => obstacle(y, x),
    }
}

pub fn process(_input: &str) -> usize {
    todo!()
}
