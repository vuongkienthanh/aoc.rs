use crate::parsing::parse_input;
use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    let (mut plants, map) = parse_input(_input);
    println!("{plants:?}");

    for _ in 0..1 {
        // for _ in 0..20 {
        add_padding(&mut plants);
        let mut new_plants = VecDeque::new();
        let mut windows: VecDeque<usize> = (0..5).map(|_| plants.pop_front().unwrap()).collect();
        while let Some(p) = plants.pop_front() {
            windows.pop_front();
            windows.push_back(p);
            let no_plants_loc: Vec<usize> = windows
                .iter()
                .enumerate()
                .filter_map(|(i, x)| (x % 2 == 0).then_some(i))
                .collect();
            let current = windows[2];
            if let Some(i) = map.get(&no_plants_loc) {
                new_plants.push_back(current.strict_add_signed(*i));
            } else {
                new_plants.push_back(current);
            }
        }
        plants = new_plants;
    }
    println!("{plants:?}");

    0
}

fn add_padding(plants: &mut VecDeque<usize>) {
    let first_plant_loc = plants
        .iter()
        .enumerate()
        .find_map(|(i, x)| ((x % 2) == 1).then_some(i))
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
// fn remove_padding(plants: &mut VecDeque<usize>) {
// }

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
