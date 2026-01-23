use crate::DIRS;
use intcode::{Computer, parse};
use std::collections::BTreeMap;

type Point = (isize, isize);

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let comp = Computer::new(input);
    let mut seen: BTreeMap<Point, usize> = BTreeMap::new();
    seen.insert((0, 0), 0);
    let mut ans = usize::MAX;

    let mut current = vec![(comp, (0, 0), 0)];
    while !current.is_empty() {
        let mut new = vec![];

        for (comp, loc, step) in current {
            for ((x, y), i) in DIRS.iter().zip(1..5) {
                let mut comp = comp.clone();
                comp.input(i);
                match comp.long_run().output() {
                    0 => (),
                    1 => {
                        let new_loc = (loc.0 + x, loc.1 + y);
                        let new_step = step + 1;
                        if seen.get(&new_loc).cloned().unwrap_or(usize::MAX) > new_step {
                            new.push((comp, new_loc, new_step));
                            seen.insert(new_loc, new_step);
                        }
                    }
                    2 => {
                        ans = ans.min(step + 1);
                    }
                    _ => panic!("should be 0 1 2"),
                }
            }
        }

        current = new;
    }

    ans
}
