use crate::DIRS;
use intcode::{Computer, parse};
use std::collections::BTreeSet;

type Point = (isize, isize);

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let comp = Computer::new(input);
    let mut map: BTreeSet<Point> = BTreeSet::new();
    map.insert((0, 0));
    let mut oxy = (0, 0);

    let mut current = vec![(comp, (0, 0))];
    while !current.is_empty() {
        let mut new = vec![];

        for (comp, loc) in current {
            for ((x, y), i) in DIRS.iter().zip(1..5) {
                let mut comp = comp.clone();
                comp.input(i);
                match comp.long_run().output() {
                    0 => (),
                    1 => {
                        let new_loc = (loc.0 + x, loc.1 + y);
                        if map.insert(new_loc) {
                            new.push((comp, new_loc));
                        }
                    }
                    2 => {
                        let new_loc = (loc.0 + x, loc.1 + y);
                        if map.insert(new_loc) {
                            new.push((comp, new_loc));
                        }
                        oxy = new_loc;
                    }
                    _ => panic!("should be 0 1 2"),
                }
            }
        }

        current = new;
    }
    let mut current = vec![oxy];
    let mut seen = BTreeSet::new();
    seen.insert(oxy);
    let mut minute = 0;
    loop {
        let mut new= vec![];
        for (a, b) in current {
            for (x, y) in DIRS {
                let new_loc = (a + x, b + y);
                if map.contains(&new_loc) && seen.insert(new_loc) {
                    new.push(new_loc);
                }
            }
        }
        current = new;
        if current.is_empty() {
            break;
        } else {
            minute += 1;
        }
    }

    minute
}
