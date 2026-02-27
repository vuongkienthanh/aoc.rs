#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Cell {
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
// ####1#1#1#11#
// #89.0.1.2.34#
// ###1#3#5#7###
//   #0#2#4#6#
//   #########

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
    for r in [[9, 8], [13, 14], [1, 0], [3, 2], [5, 4], [7, 6]] {
        if let Some(i) = r.into_iter().find(|x| !matches!(locations[x], Cell::E)) {
            ans.push(i);
        }
    }
    ans.extend(
        [10, 11, 12]
            .into_iter()
            .filter(|x| !matches!(locations[x], Cell::E)),
    );
    ans
}
// fn get_movable_to(from: usize) -> Vec<usize> {
//     let mut ans = vec![];
//
//     ans
// }

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

fn step(from: usize, to: usize) -> usize {
    match (from, to) {
        //
        (0, 9) => 3,
        (0, 10) => 3,
        (0, 11) => 5,
        (0, 12) => 7,
        (0, 13) => 9,
        //
        (2, 9) => 5,
        (2, 10) => 3,
        (2, 11) => 3,
        (2, 12) => 5,
        (2, 13) => 7,
        //
        (4, 9) => 7,
        (4, 10) => 5,
        (4, 11) => 3,
        (4, 12) => 3,
        (4, 13) => 5,
        //
        (6, 9) => 9,
        (6, 10) => 7,
        (6, 11) => 5,
        (6, 12) => 3,
        (6, 13) => 3,
        // edge hall
        (x, 8) => step(x, 9) + 1,
        (x, 14) => step(x, 13) + 1,
        // outer rooms
        (1, y) => step(0, y) - 1,
        (3, y) => step(2, y) - 1,
        (5, y) => step(4, y) - 1,
        (7, y) => step(6, y) - 1,
        //
        (x, y) if x >= 8 && y >= 8 => panic!("hall to hall"),
        // reverse
        (x, y) => step(y, x),
    }
}

pub fn process(_input: &str) -> usize {
    todo!()
}

