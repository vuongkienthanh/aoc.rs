use crate::parsing::{Item, Point, parse_input};
// use crate::walk::{Cache, WalkResult, walk};
use std::collections::{HashMap, HashSet};
use crate::walk2::walk;

pub fn process(_input: &str) -> usize {
    let (g, number_locations) = parse_input(_input);
    // println!("{g:?}");
    // println!("number_locations: {number_locations:>}");

    // type State = (Vec<Point>, usize);
    // let mut cache = Cache::new();
    let mut minmap: HashMap<char, HashMap<char, usize>> = HashMap::new();
    let keys : Vec<char> = number_locations.keys().copied().collect();

    for from in keys {
        minmap.insert( from, walk(from, &g, &number_locations));
    }

    println!("{minmap:?}");
    todo!("itertools")
}

// fn count_step(v: Vec<(Point, usize)>, cache: &Cache) -> usize {
//     v.into_iter()
//         .map(|(p, i)| match cache.get(&p).unwrap()[i] {
//             WalkResult::DeadEnd | WalkResult::Unknown => panic!("should have walked"),
//             WalkResult::Dst { step, to: _ } => step,
//         })
//         .sum()
// }
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"###########
#0.1.....2#
#.#######.#
#4.......3#
###########"#
    }
    #[rstest]
    fn test_process_1(fixture: &str) {
        assert_eq!(process(fixture), 14);
    }
}
