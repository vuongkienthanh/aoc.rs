use crate::parsing::{Item, Point, parse_input};
use crate::walk::{Cache, WalkResult, walk};
use std::collections::{HashMap, HashSet};

pub fn process(_input: &str) -> usize {
    let (g, number_locations) = parse_input(_input);
    println!("{g:?}");
    println!("number_locations: {number_locations:>}");

    type State = (Point, Vec<Point>, usize);
    let mut cache = Cache::new();
    let mut minmap: HashMap<char, HashMap<char, usize>> = HashMap::new();

    for (c, p) in number_locations.iter().cloned() {
        let mut v: Vec<State> = vec![];
        v.push((p, vec![], 0));
        while !v.is_empty() {
            let mut new_v = vec![];
            while let Some((curr, path, steps)) = v.pop() {

                if let Item::Number(d) = g[curr]
                    && d != c
                {
                    minmap
                        .entry(c)
                        .or_default()
                        .entry(d)
                        .and_modify(|x| *x = x.min(steps))
                        .or_insert(steps);
                }

                for (i, walk_result) in walk(curr, &g, &mut cache).into_iter().enumerate() {
                    match walk_result {
                        WalkResult::DeadEnd => (),
                        WalkResult::Dst { step: _, to } => {
                            if !path.contains(&(curr, i)) {
                                let mut new_path = path.clone();
                                new_path.push((curr, i));
                                new_v.push((to, number_seen.clone(), new_path))
                            }
                        }
                    }
                }
            }

            v.extend(new_v);
            println!("{v:?}");
            break;
        }
    }

    todo!()
}

fn count_step(v: Vec<(Point, usize)>, cache: &Cache) -> usize {
    v.into_iter()
        .map(|(p, i)| match cache.get(&p).unwrap()[i] {
            WalkResult::DeadEnd | WalkResult::Unknown => panic!("should have walked"),
            WalkResult::Dst { step, to: _ } => step,
        })
        .sum()
}
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
