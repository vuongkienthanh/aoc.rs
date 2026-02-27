// ####1#1#1#11#
// #89.0.1.2.34#
// ###1#3#5#7###
//   #0#2#4#6#
//   #########

use fxhash::FxHashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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
impl Cell {
    fn room(&self) -> [usize; 2] {
        match self {
            Cell::A => [1, 0],
            Cell::B => [3, 2],
            Cell::C => [5, 4],
            Cell::D => [7, 6],
            Cell::E => panic!(),
        }
    }
    fn left(&self) -> Vec<usize> {
        match self {
            Cell::A => vec![8, 9],
            Cell::B => vec![8, 9, 10],
            Cell::C => vec![8, 9, 10, 11],
            Cell::D => vec![8, 9, 10, 11, 12],
            Cell::E => panic!(),
        }
    }
    fn right(&self) -> Vec<usize> {
        match self {
            Cell::A => vec![14, 13, 12, 11, 10],
            Cell::B => vec![14, 13, 12, 11],
            Cell::C => vec![14, 13, 12],
            Cell::D => vec![14, 13],
            Cell::E => panic!(),
        }
    }
    fn score(&self) -> usize {
        match self {
            Cell::A => 1,
            Cell::B => 10,
            Cell::C => 100,
            Cell::D => 1000,
            Cell::E => panic!(),
        }
    }
}

#[derive(Debug)]
struct State {
    locations: [Cell; 15],
    score: usize,
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
            score: 0,
        }
    }
}
impl State {
    fn is_done(&self) -> bool {
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

    fn next(&self) -> Vec<State> {
        let mut ans = vec![];
        for from in get_from(&self.locations) {
            for to in get_to(&self.locations, from) {
                let mut locations = self.locations;
                locations.swap(from, to);
                let score = self.score + step(from, to) * locations[to].score();
                ans.push(State { locations, score });
            }
        }
        ans
    }
}

fn get_from(locations: &[Cell; 15]) -> Vec<usize> {
    let mut ans = vec![];
    for cell in [Cell::A, Cell::B, Cell::C, Cell::D] {
        let rooms = cell.room();
        if rooms
            .iter()
            .any(|x| locations[*x] != cell && locations[*x] != Cell::E)
            && let Some(i) = rooms.into_iter().find(|x| locations[*x] != Cell::E)
        {
            ans.push(i);
        }
    }
    for r in [[9, 8], [13, 14]] {
        if let Some(i) = r.into_iter().find(|x| locations[*x] != Cell::E) {
            ans.push(i);
        }
    }
    ans.extend(
        [10, 11, 12]
            .into_iter()
            .filter(|x| locations[*x] != Cell::E),
    );
    ans
}

fn get_to(locations: &[Cell; 15], from: usize) -> Vec<usize> {
    let mut ans = vec![];

    match from {
        0..8 => {
            let cell = [Cell::A, Cell::B, Cell::C, Cell::D]
                .into_iter()
                .find(|x| x.room().contains(&from))
                .unwrap();
            for i in cell
                .left()
                .into_iter()
                .take_while(|x| locations[*x] == Cell::E)
            {
                ans.push(i);
            }
            for i in cell
                .right()
                .into_iter()
                .take_while(|x| locations[*x] == Cell::E)
            {
                ans.push(i);
            }
            // if let Some(i) = cell.left().into_iter().find(|x| locations[*x] == Cell::E) {
            //     ans.push(i);
            // }
            // if let Some(i) = cell.right().into_iter().find(|x| locations[*x] == Cell::E) {
            //     ans.push(i);
            // }
        }
        _ => {
            let amphipod = locations[from];
            let rooms = amphipod.room();
            if rooms
                .iter()
                .all(|x| locations[*x] == amphipod || locations[*x] == Cell::E)
            {
                let to = rooms
                    .into_iter()
                    .rfind(|x| locations[*x] == Cell::E)
                    .unwrap();
                if obstacle(from, to)
                    .into_iter()
                    .all(|x| locations[x] == Cell::E)
                {
                    ans.push(to);
                }
            }
        }
    }
    ans
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
    let mut current = vec![State::from(_input)];
    let mut seen: FxHashMap<[Cell; 15], usize> = FxHashMap::default();
    seen.insert(current.first().unwrap().locations, 0);
    let mut ans = usize::MAX;
    while !current.is_empty() {
        let mut new = vec![];

        for state in current {
            // println!("from state {:?}", state.locations);
            if state.is_done() {
                ans = ans.min(state.score);
                continue;
            }
            for next_state in state.next() {
                if let Some(s) = seen.get(&next_state.locations) {
                    if next_state.score < *s {
                        seen.insert(next_state.locations, next_state.score);
                        // println!("to state {:?}", next_state.locations);
                        new.push(next_state);
                    }
                } else {
                    seen.insert(next_state.locations, next_state.score);
                    // println!("to state {:?}", next_state.locations);
                    new.push(next_state);
                }
            }
        }

        current = new;
    }
    ans
}
