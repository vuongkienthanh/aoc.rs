pub mod part1;
pub mod part2;
use fxhash::FxHashMap;
use std::ops::{Range, RangeInclusive};

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
    const fn score(&self) -> usize {
        match self {
            Cell::A => 1,
            Cell::B => 10,
            Cell::C => 100,
            Cell::D => 1000,
            Cell::E => panic!(),
        }
    }
}
struct Map<const N: usize>;

impl<const N: usize> Map<N> {
    const LEN: usize = (N - 7) / 4;
    const HALL_1: usize = Self::LEN * 4;
    const HALL_2: usize = Self::HALL_1 + 1;
    const HALL_3: usize = Self::HALL_2 + 1;
    const HALL_4: usize = Self::HALL_3 + 1;
    const HALL_5: usize = Self::HALL_4 + 1;
    const HALL_6: usize = Self::HALL_5 + 1;
    const HALL_7: usize = Self::HALL_6 + 1;
    const RH_0: [(usize, usize); 8] = [
        (0, Self::HALL_2),
        (0, Self::HALL_3),
        (Self::LEN, Self::HALL_3),
        (Self::LEN, Self::HALL_4),
        (Self::LEN * 2, Self::HALL_4),
        (Self::LEN * 2, Self::HALL_5),
        (Self::LEN * 3, Self::HALL_5),
        (Self::LEN * 3, Self::HALL_6),
    ];
    const RH_1: [(usize, usize); 6] = [
        (0, Self::HALL_4),
        (Self::LEN, Self::HALL_2),
        (Self::LEN, Self::HALL_5),
        (Self::LEN * 2, Self::HALL_3),
        (Self::LEN * 2, Self::HALL_6),
        (Self::LEN * 3, Self::HALL_4),
    ];
    const RH_2: [(usize, usize); 4] = [
        (0, Self::HALL_5),
        (Self::LEN, Self::HALL_6),
        (Self::LEN * 2, Self::HALL_2),
        (Self::LEN * 3, Self::HALL_3),
    ];
    const RH_3: [(usize, usize); 2] = [(0, Self::HALL_6), (Self::LEN * 3, Self::HALL_2)];
    const HR_0: [(usize, usize); 8] = [
        (Self::HALL_2, 0),
        (Self::HALL_3, 0),
        (Self::HALL_3, Self::LEN),
        (Self::HALL_4, Self::LEN),
        (Self::HALL_4, Self::LEN * 2),
        (Self::HALL_5, Self::LEN * 2),
        (Self::HALL_5, Self::LEN * 3),
        (Self::HALL_6, Self::LEN * 3),
    ];
    const HR_H3: [(usize, usize); 2] = [(Self::HALL_2, Self::LEN), (Self::HALL_4, 0)];
    const HR_H4: [(usize, usize); 2] = [(Self::HALL_3, Self::LEN * 2), (Self::HALL_5, Self::LEN)];
    const HR_H5: [(usize, usize); 2] =
        [(Self::HALL_4, Self::LEN * 3), (Self::HALL_6, Self::LEN * 2)];
    const HR_H34: [(usize, usize); 2] = [(Self::HALL_2, Self::LEN * 2), (Self::HALL_5, 0)];
    const HR_H45: [(usize, usize); 2] = [(Self::HALL_3, Self::LEN * 3), (Self::HALL_6, Self::LEN)];
    const HR_H345: [(usize, usize); 2] = [(Self::HALL_2, Self::LEN * 3), (Self::HALL_6, 0)];

    const fn room(&self, cell: &Cell) -> Range<usize> {
        match cell {
            Cell::A => 0..Self::LEN,
            Cell::B => Self::LEN..Self::LEN * 2,
            Cell::C => Self::LEN * 2..Self::LEN * 3,
            Cell::D => Self::LEN * 3..Self::LEN * 4,
            Cell::E => panic!(),
        }
    }
    const fn left(&self, cell: &Cell) -> RangeInclusive<usize> {
        match cell {
            Cell::A => Self::HALL_1..=Self::HALL_2,
            Cell::B => Self::HALL_1..=Self::HALL_3,
            Cell::C => Self::HALL_1..=Self::HALL_4,
            Cell::D => Self::HALL_1..=Self::HALL_5,
            Cell::E => panic!(),
        }
    }
    const fn right(&self, cell: &Cell) -> RangeInclusive<usize> {
        match cell {
            Cell::A => Self::HALL_3..=Self::HALL_7,
            Cell::B => Self::HALL_4..=Self::HALL_7,
            Cell::C => Self::HALL_5..=Self::HALL_7,
            Cell::D => Self::HALL_6..=Self::HALL_7,
            Cell::E => panic!(),
        }
    }
    const fn left_hall(&self) -> [usize; 2] {
        [Self::HALL_2, Self::HALL_1]
    }
    const fn right_hall(&self) -> [usize; 2] {
        [Self::HALL_6, Self::HALL_7]
    }
    const fn mid_hall(&self) -> [usize; 3] {
        [Self::HALL_3, Self::HALL_4, Self::HALL_5]
    }
    fn step(&self, from: usize, to: usize) -> usize {
        match (from, to) {
            (x, y) if Self::RH_0.contains(&(x, y)) => Self::LEN + 1,
            (x, y) if Self::RH_1.contains(&(x, y)) => Self::LEN + 3,
            (x, y) if Self::RH_2.contains(&(x, y)) => Self::LEN + 5,
            (x, y) if Self::RH_3.contains(&(x, y)) => Self::LEN + 7,
            (x, y) if y == Self::HALL_1 => self.step(x, Self::HALL_2) + 1,
            (x, y) if y == Self::HALL_7 => self.step(x, Self::HALL_6) + 1,
            (x, y) if (1..Self::LEN).contains(&x) => self.step(0, y) - x,
            (x, y) if (Self::LEN + 1..Self::LEN * 2).contains(&x) => {
                self.step(Self::LEN, y) + Self::LEN - x
            }
            (x, y) if (Self::LEN * 2 + 1..Self::LEN * 3).contains(&x) => {
                self.step(Self::LEN * 2, y) + Self::LEN * 2 - x
            }
            (x, y) if (Self::LEN * 3 + 1..Self::LEN * 4).contains(&x) => {
                self.step(Self::LEN * 3, y) + Self::LEN * 3 - x
            }
            (x, y) => self.step(y, x),
        }
    }
    fn obstacle(&self, from: usize, to: usize) -> Vec<usize> {
        match (from, to) {
            (x, y) if Self::HR_0.contains(&(x, y)) => vec![],
            (x, y) if Self::HR_H3.contains(&(x, y)) => vec![Self::HALL_3],
            (x, y) if Self::HR_H4.contains(&(x, y)) => vec![Self::HALL_4],
            (x, y) if Self::HR_H5.contains(&(x, y)) => vec![Self::HALL_5],
            (x, y) if Self::HR_H34.contains(&(x, y)) => vec![Self::HALL_3, Self::HALL_4],
            (x, y) if Self::HR_H45.contains(&(x, y)) => vec![Self::HALL_4, Self::HALL_5],
            (x, y) if Self::HR_H345.contains(&(x, y)) => {
                vec![Self::HALL_3, Self::HALL_4, Self::HALL_5]
            }
            (x, y) if x == Self::HALL_1 => self.obstacle(Self::HALL_2, y),
            (x, y) if x == Self::HALL_7 => self.obstacle(Self::HALL_6, y),
            (x, y) if (1..Self::LEN).contains(&y) => self.obstacle(x, 0),
            (x, y) if (Self::LEN + 1..Self::LEN * 2).contains(&y) => self.obstacle(x, Self::LEN),
            (x, y) if (Self::LEN * 2 + 1..Self::LEN * 3).contains(&y) => {
                self.obstacle(x, Self::LEN * 2)
            }
            (x, y) if (Self::LEN * 3 + 1..Self::LEN * 4).contains(&y) => {
                self.obstacle(x, Self::LEN * 3)
            }
            (x, y) => panic!("from {x} to {y}"),
        }
    }
    fn get_from(&self, locations: &[Cell]) -> Vec<usize> {
        let mut ans = vec![];
        for cell in [Cell::A, Cell::B, Cell::C, Cell::D] {
            if !self
                .room(&cell)
                .all(|x| locations[x] == cell || locations[x] == Cell::E)
                && let Some(i) = self.room(&cell).rfind(|x| locations[*x] != Cell::E)
            {
                ans.push(i);
            }
        }
        for r in [self.left_hall(), self.right_hall()] {
            if let Some(i) = r.into_iter().find(|x| locations[*x] != Cell::E) {
                ans.push(i);
            }
        }
        ans.extend(
            self.mid_hall()
                .into_iter()
                .filter(|x| locations[*x] != Cell::E),
        );
        ans
    }
    fn get_to(&self, locations: &[Cell], from: usize) -> Vec<usize> {
        let mut ans = vec![];
        if let Some(room) = [Cell::A, Cell::B, Cell::C, Cell::D]
            .into_iter()
            .find(|x| self.room(x).contains(&from))
        {
            for i in self
                .left(&room)
                .rev()
                .take_while(|x| locations[*x] == Cell::E)
            {
                ans.push(i);
            }
            for i in self.right(&room).take_while(|x| locations[*x] == Cell::E) {
                ans.push(i);
            }
        } else {
            let cell = locations[from];
            if self
                .room(&cell)
                .all(|x| locations[x] == cell || locations[x] == Cell::E)
            {
                let to = self.room(&cell).find(|x| locations[*x] == Cell::E).unwrap();
                if self
                    .obstacle(from, to)
                    .into_iter()
                    .all(|x| locations[x] == Cell::E)
                {
                    ans.push(to);
                }
            }
        }

        ans
    }
}

#[derive(Debug)]
struct State<const N: usize> {
    locations: [Cell; N],
    score: usize,
}

impl<const N: usize> State<N> {
    const LEN: usize = (N - 7) / 4;
    fn is_done(&self) -> bool {
        [Cell::A, Cell::B, Cell::C, Cell::D]
            .into_iter()
            .enumerate()
            .all(|(i, cell)| {
                self.locations[Self::LEN * i..Self::LEN * (i + 1)]
                    .iter()
                    .all(|x| *x == cell)
            })
    }
    fn next(&self) -> Vec<State<N>> {
        let map = Map::<N>;
        let mut ans = vec![];
        for from in map.get_from(&self.locations) {
            for to in map.get_to(&self.locations, from) {
                let mut locations = self.locations;
                locations.swap(from, to);
                let score = self.score + map.step(from, to) * locations[to].score();
                ans.push(State { locations, score });
            }
        }
        ans
    }
}

fn run<const N: usize>(initial_state: State<N>) -> usize {
    let mut current = vec![initial_state];
    let mut seen: FxHashMap<[Cell; N], usize> = FxHashMap::default();
    seen.insert(current.first().unwrap().locations, 0);
    let mut ans = usize::MAX;
    while !current.is_empty() {
        let mut new = vec![];

        for state in current {
            if state.is_done() {
                ans = ans.min(state.score);
                continue;
            }
            for next_state in state.next() {
                match seen.get(&next_state.locations) {
                    None => {
                        seen.insert(next_state.locations, next_state.score);
                        new.push(next_state);
                    }
                    Some(s) if next_state.score < *s => {
                        seen.insert(next_state.locations, next_state.score);
                        new.push(next_state);
                    }
                    _ => (),
                }
            }
        }

        current = new;
    }
    ans
}
