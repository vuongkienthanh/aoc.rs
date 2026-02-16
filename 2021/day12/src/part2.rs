use crate::build_map;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let map = build_map(input);
    let mut current = vec![(vec![], "start", false)];
    let mut ans = 0;
    while !current.is_empty() {
        let mut new = vec![];
        for (small_caves, last, small_cave_twice) in current {
            for nxt_loc in map.get(last).unwrap() {
                if *nxt_loc == "start" {
                    continue;
                }
                if *nxt_loc == "end" {
                    ans += 1;
                    continue;
                }
                if nxt_loc.chars().next().unwrap().is_ascii_lowercase() {
                    if small_caves.contains(nxt_loc) {
                        if !small_cave_twice {
                            let mut small_caves = small_caves.clone();
                            small_caves.push(nxt_loc);
                            new.push((small_caves, *nxt_loc, true));
                        }
                    } else {
                        let mut small_caves = small_caves.clone();
                        small_caves.push(nxt_loc);
                        new.push((small_caves, *nxt_loc, small_cave_twice));
                    }
                } else {
                    new.push((small_caves.clone(), *nxt_loc, small_cave_twice));
                }
            }
        }

        current = new;
    }

    ans
}
