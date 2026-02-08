use crate::{parse_block, parse_inventory};
use fxhash::FxHashSet;
use intcode::{Computer, RunResult, parse};
use itertools::Itertools;

fn opposite(d: &str) -> &str {
    match d {
        "east" => "west",
        "west" => "east",
        "south" => "north",
        "north" => "south",
        _ => panic!(),
    }
}

#[allow(unused_assignments)]
pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut comp = Computer::new(input);
    let start_state = comp.ascii("");
    let (mut loc, mut avail_doors, mut item) = parse_block(&start_state);
    let target = "Security Checkpoint";
    let mut path_to_target: Vec<String> = vec![];
    let mut dir_to_floor = String::new();
    let ignore = [
        "molten lava",
        "infinite loop",
        "escape pod",
        "giant electromagnet",
        "photons",
    ];
    let mut seen: FxHashSet<Vec<String>> = FxHashSet::default();
    let mut path: Vec<String> = vec![];

    loop {
        let mut output = String::new();
        for door in avail_doors {
            if let Some(d) = path.last() {
                if opposite(d.as_str()) == door {
                    continue;
                }
            }
            path.push(door.clone());
            if seen.insert(path.clone()) {
                output = comp.ascii(&door);
                break;
            } else {
                path.pop();
            }
        }
        if !output.is_empty() {
            (loc, avail_doors, item) = parse_block(&output);
            if !item.is_empty() && !ignore.contains(&item.as_str()) {
                let cmd = format!("take {item}");
                comp.ascii(&cmd);
            }
            if loc == target {
                path_to_target = path.clone();
                let d = path.pop().unwrap();
                let b = opposite(d.as_str());
                dir_to_floor = avail_doors.into_iter().find(|door| door != b).unwrap();
                output = comp.ascii(b);
                (loc, avail_doors, item) = parse_block(&output);
            }
        } else {
            if let Some(d) = path.pop() {
                let b = opposite(d.as_str());
                output = comp.ascii(b);
                (loc, avail_doors, item) = parse_block(&output);
            } else {
                break;
            }
        }
    }
    for path in path_to_target {
        comp.ascii(&path);
    }

    let output = comp.ascii("inv");
    let inventory = parse_inventory(&output);

    for i in 1..inventory.len() {
        for v in inventory.iter().combinations(i) {
            let mut comp = comp.clone();
            for item in &v {
                let cmd = format!("drop {item}");
                comp.ascii(&cmd);
            }
            let output = comp.ascii(&dir_to_floor);
            if !output.contains("ejected") {
                println!("{v:?}");
                println!("{output}");
                return 1;
            }
        }
    }
    0
}