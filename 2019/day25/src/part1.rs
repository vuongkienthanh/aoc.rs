use crate::{parse_block, parse_inventory};
use fxhash::FxHashSet;
use intcode::{Computer, RunResult, parse};

type State = (String, Vec<String>);

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut comp = Computer::new(input);
    let start_state = comp.ascii("");
    println!("{start_state}");
    let (loc, doors, items) = parse_block(&start_state);
    println!("{loc:?} {doors:?} {items:?}");

    let mut seen: FxHashSet<State> = FxHashSet::default();
    let mut current = vec![(comp, loc, doors, items)];

    loop {
        let mut new = vec![];
        println!("current len = {}", current.len());
        for (mut comp, loc, avail_doors, avail_items) in current {
            let inv = comp.ascii("inv");
            println!("inv= {inv}");
            let inventory = parse_inventory(&inv);

            if seen.insert((loc.clone(), inventory.clone())) {
                for door in &avail_doors {
                    let mut comp = comp.clone();
                    let new_state = comp.ascii(door);
                    println!("new_state= {new_state}");
                    let (loc, doors, items) = parse_block(&new_state);
                    new.push((comp, loc, doors, items));
                }
                for item in &avail_items {
                    if ["molten lava", "infinite loop", "escape pod"].contains(&item.as_str()) {
                        continue;
                    }
                    let mut comp = comp.clone();
                    let cmd = format!("take {item}");
                    let taken = comp.ascii(&cmd);
                    println!("taken= {taken}");
                    let items = avail_items
                        .iter()
                        .filter(|x| *x != item)
                        .map(|x| x.to_string())
                        .collect();
                    new.push((comp, loc.clone(), avail_doors.clone(), items));
                }
                for item in inventory {
                    let mut comp = comp.clone();
                    let cmd = format!("drop {item}");
                    let drop = comp.ascii(&cmd);
                    println!("{drop}");
                    let mut items = avail_items.clone();
                    items.push(item.to_string());
                    new.push((comp, loc.clone(), avail_doors.clone(), items));
                }
            }
        }
        let mut new2 = vec![];
        for (mut comp, a, b, c) in new {
            match comp.long_run() {
                RunResult::Halt => {
                    println!("{a} {b:?} {c:?}");
                }
                RunResult::WaitingInput => new2.push((comp, a, b, c)),
                RunResult::Output(x) => {
                    panic!("should not have any output, see {}", x as u8 as char)
                }
            }
        }
        current = new2;

        if current.is_empty() {
            break;
        }
    }

    0
}
