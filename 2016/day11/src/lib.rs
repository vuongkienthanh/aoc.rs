pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::Item;
use std::collections::{BTreeSet, HashMap};

pub type PairState = (BTreeSet<(usize, usize, usize)>, usize);

#[derive(Debug, Clone)]
pub struct State<'a> {
    map: Vec<Vec<Item<'a>>>,
    elevator: usize,
}

impl<'a> State<'a> {
    fn to_pairstate(&self) -> PairState {
        let mut m: HashMap<&str, (usize, usize)> = HashMap::new();
        for (i, floor) in self.map.iter().enumerate() {
            for item in floor {
                match item {
                    Item::Generator(x) => m.entry(x).or_default().0 = i,
                    Item::Microchip(x) => m.entry(x).or_default().1 = i,
                }
            }
        }
        let m2: HashMap<(usize, usize), usize> =
            m.into_values().fold(HashMap::new(), |mut acc, ele| {
                *acc.entry(ele).or_default() += 1;
                acc
            });
        (
            m2.into_iter().map(|((g, m), c)| (g, m, c)).collect(),
            self.elevator,
        )
    }
    fn is_all_fourth_floor(&self) -> bool {
        let l = self.map.len();
        self.map.iter().take(l - 1).all(|x| x.is_empty())
    }
    fn get_items_on_floor(&'a self, i: usize) -> (Vec<&'a str>, Vec<&'a str>) {
        let mut generators = vec![];
        let mut microchips = vec![];
        let floor = self.map.get(i).unwrap();
        for item in floor {
            match item {
                Item::Generator(x) => generators.push(*x),
                Item::Microchip(x) => microchips.push(*x),
            }
        }
        (generators, microchips)
    }
    fn is_floor_ok(&self, i: usize) -> bool {
        let (generators, mut microchips) = self.get_items_on_floor(i);
        if generators.is_empty() {
            return true;
        }
        while let Some(mc) = microchips.pop() {
            if !generators.contains(&mc) {
                return false;
            }
        }
        true
    }
    fn possible_next_states(self) -> Vec<State<'a>> {
        fn one_item<'a>(state: &State<'a>, next_elevator: usize) -> Vec<State<'a>> {
            let mut ans = vec![];
            for i in 0..state.map[state.elevator].len() {
                let mut new_map = state.map.clone();
                let item = new_map[state.elevator].remove(i);
                new_map[next_elevator].push(item);
                let new_state = State {
                    map: new_map,
                    elevator: next_elevator,
                };
                if new_state.is_floor_ok(state.elevator) && new_state.is_floor_ok(next_elevator) {
                    ans.push(new_state);
                }
            }
            ans
        }

        fn two_items<'a>(state: &State<'a>, next_elevator: usize) -> Vec<State<'a>> {
            let mut ans = vec![];
            for i in 0..state.map[state.elevator].len() - 1 {
                for j in i + 1..state.map[state.elevator].len() {
                    let mut new_map = state.map.clone();
                    let item = new_map[state.elevator].remove(j);
                    new_map[next_elevator].push(item);
                    let item = new_map[state.elevator].remove(i);
                    new_map[next_elevator].push(item);
                    let new_state = State {
                        map: new_map,
                        elevator: next_elevator,
                    };
                    if new_state.is_floor_ok(state.elevator) && new_state.is_floor_ok(next_elevator)
                    {
                        ans.push(new_state);
                    }
                }
            }
            ans
        }

        let mut ans = vec![];
        let mut moving_up = vec![];
        if self.elevator < self.map.len() - 1 {
            let next_elevator = self.elevator + 1;
            moving_up.extend(two_items(&self, next_elevator));
            if moving_up.is_empty() {
                moving_up.extend(one_item(&self, next_elevator));
            }
        }
        let mut moving_down = vec![];
        let first_occupied_floor = self
            .map
            .iter()
            .enumerate()
            .find(|(_, f)| !f.is_empty())
            .map(|(i, _)| i)
            .unwrap();
        if self.elevator > 0 && (self.elevator - 1) >= first_occupied_floor {
            let next_elevator = self.elevator - 1;
            moving_down.extend(one_item(&self, next_elevator));
            if moving_down.is_empty() {
                moving_down.extend(two_items(&self, next_elevator))
            }
        }

        ans.extend(moving_up);
        ans.extend(moving_down);
        ans
    }
}
