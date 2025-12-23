use crate::parsing::{Item, parse_input};
use crate::{PairState, State};
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut heap = Vec::new();
    let mut seen: HashSet<PairState> = HashSet::new();
    let mut initial_state = State {
        map: input,
        elevator: 0,
    };

    initial_state.map[0].push(Item::Generator("elerium"));
    initial_state.map[0].push(Item::Microchip("elerium"));
    initial_state.map[0].push(Item::Generator("dilithium"));
    initial_state.map[0].push(Item::Microchip("dilithium"));

    heap.push(initial_state.clone());
    seen.insert(initial_state.to_pairstate());

    let mut step = 0;
    loop {
        let mut new_heap = vec![];
        for state in heap {
            for new_state in state.possible_next_states() {
                if new_state.is_all_fourth_floor() {
                    return step + 1;
                }
                if seen.insert(new_state.to_pairstate()) {
                    new_heap.push(new_state);
                }
            }
        }

        heap = new_heap;
        step += 1;
    }
}
