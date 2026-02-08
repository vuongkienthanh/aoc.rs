use crate::parsing::{Map, parse_input};
use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    let (plants, map) = parse_input(_input);
    run(plants, map, 20)
}

pub fn run(mut plants: VecDeque<usize>, map: Map, count: usize) -> usize {
    for _ in 0..count {
        add_padding(&mut plants);
        let mut new_plants = VecDeque::new();
        let mut windows: VecDeque<usize> = (0..4).map(|_| plants.pop_front().unwrap()).collect();
        while let Some(p) = plants.pop_front() {
            windows.push_back(p);
            let no_plants_loc: Vec<usize> = windows
                .iter()
                .enumerate()
                .filter_map(|(i, x)| (x % 2 == 0).then_some(i))
                .collect();
            let mut i = if map.contains(&no_plants_loc) { 1 } else { 0 };
            if windows[2] > 1 {
                i += 2;
            }
            new_plants.push_back(i);
            windows.pop_front();
        }
        plants = new_plants;
    }
    let right_shift = plants
        .iter()
        .enumerate()
        .find_map(|(i, x)| (*x > 1).then_some(i))
        .unwrap() as isize;
    plants
        .into_iter()
        .enumerate()
        .flat_map(|(i, x)| (x % 2 == 1).then_some(i as isize - right_shift))
        .sum::<isize>() as usize
}

fn add_padding(plants: &mut VecDeque<usize>) {
    let first_plant_loc = plants
        .iter()
        .enumerate()
        .find_map(|(i, x)| (*x > 1).then_some(i))
        .unwrap();
    for _ in 0..3usize.saturating_sub(first_plant_loc) {
        plants.push_front(0);
    }
    let last_plant_loc = plants
        .iter()
        .rev()
        .enumerate()
        .find_map(|(i, x)| ((x % 2) == 1).then_some(i))
        .unwrap();
    for _ in 0..3usize.saturating_sub(last_plant_loc) {
        plants.push_back(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 325);
    }
}
