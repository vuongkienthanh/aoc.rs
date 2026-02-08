use crate::parsing::parse_input;
use crate::{PairState, State};
use std::collections::HashSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut heap = Vec::new();
    let mut seen: HashSet<PairState> = HashSet::new();
    let initial_state = State {
        map: input,
        elevator: 0,
    };
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant."#
    }
    #[rstest]
    fn test_process(fixture: &str) {
        assert_eq!(process(fixture), 11);
    }
}
