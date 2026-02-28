pub mod part1;
pub mod part2;
use fxhash::FxHashMap;
use std::ops::Range;

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
trait Map {
    fn room(&self, cell: &Cell) -> Range<usize>;
    fn left(&self, cell: &Cell) -> impl Iterator<Item = usize>;
    fn right(&self, cell: &Cell) -> impl Iterator<Item = usize>;
    fn left_hall(&self) -> [usize; 2];
    fn right_hall(&self) -> [usize; 2];
    fn mid_hall(&self) -> [usize; 3];
    fn step(&self, from: usize, to: usize) -> usize;
    fn obstacle(&self, from: usize, to: usize) -> Vec<usize>;
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
        if let Some(cell) = [Cell::A, Cell::B, Cell::C, Cell::D]
            .into_iter()
            .find(|x| self.room(x).contains(&from))
        {
            for i in self.left(&cell).take_while(|x| locations[*x] == Cell::E) {
                ans.push(i);
            }
            for i in self.right(&cell).take_while(|x| locations[*x] == Cell::E) {
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
    fn next(&self, map: &impl Map) -> Vec<State<N>> {
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

fn run<const N: usize>(initial_state: State<N>, map: impl Map) -> usize {
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
            for next_state in state.next(&map) {
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
