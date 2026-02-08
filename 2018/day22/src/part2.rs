use crate::parsing::parse_input;
use crate::{Cache, Point, get_type};
use aoc_helper::adj::checked_u::adj4;
use fxhash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Equip {
    Torch,
    Gear,
    Neither,
}

const USABLE: [[Equip; 2]; 3] = [
    [Equip::Gear, Equip::Torch],
    [Equip::Gear, Equip::Neither],
    [Equip::Neither, Equip::Torch],
];

struct State(Point, Equip, usize);

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }
}
impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

type Map = FxHashMap<(Point, Equip), usize>;

fn change_equipment(actor: &State, depth: usize, target: Point, cache: &mut Cache) -> State {
    let terrain = get_type(actor.0, depth, target, cache);
    let changed = USABLE[terrain].into_iter().find(|e| e != &actor.1).unwrap();
    State(actor.0, changed, actor.2 + 7)
}

fn movable(actor: &State, dst: Point, depth: usize, target: Point, cache: &mut Cache) -> bool {
    let terrain = get_type(dst, depth, target, cache);
    USABLE[terrain].contains(&actor.1)
}

fn action(
    actor: State,
    depth: usize,
    target: Point,
    cache: &mut Cache,
    map: &mut Map,
) -> Vec<State> {
    let mut ans = vec![];

    let a2 = change_equipment(&actor, depth, target, cache);
    checked_add_new_actor(a2, &mut ans, map);

    for p in adj4(actor.0).into_iter().flatten() {
        if movable(&actor, p, depth, target, cache) {
            let a2 = State(p, actor.1, actor.2 + 1);
            checked_add_new_actor(a2, &mut ans, map);
        }
    }
    ans
}

fn checked_add_new_actor(actor: State, ans: &mut Vec<State>, map: &mut Map) {
    if let Some(st) = map.get(&(actor.0, actor.1)) {
        if actor.2 < *st {
            map.insert((actor.0, actor.1), actor.2);
            ans.push(actor);
        }
    } else {
        map.insert((actor.0, actor.1), actor.2);
        ans.push(actor);
    }
}

pub fn process(_input: &str) -> usize {
    let (depth, target) = parse_input(_input);
    let mut cache = Cache::new();

    let actor: State = State((0, 0), Equip::Torch, 0);
    let mut map = Map::default();
    map.insert((actor.0, actor.1), actor.2);

    let mut queue = BinaryHeap::from([actor]);

    while let Some(actor) = queue.pop() {
        if actor.0 == target && actor.1 == Equip::Torch {
            return actor.2;
        }
        queue.extend(action(actor, depth, target, &mut cache, &mut map));
    }

    panic!("should have an answer")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"depth: 510
target: 10,10"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 45);
    }
}
