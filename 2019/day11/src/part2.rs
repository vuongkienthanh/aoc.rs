use aoc_helper::direction::{Direction, step};
use intcode::{Computer, parse};
use std::collections::BTreeMap;

pub fn process(_input: &str) -> usize {
    let input = parse(_input);
    let mut map = BTreeMap::new();
    let (mut loc, mut dir) = ((0isize, 0isize), Direction::Up);
    map.insert(loc, 1);
    let mut comp = Computer::new(input);
    comp.append_input(1);

    while let Some(paint) = comp.long_run() {
        map.insert(loc, paint);
        let turn = comp.long_run().unwrap();
        match turn {
            0 => {
                dir = dir.turn_left();
                let (x, y) = step(dir);
                loc = (loc.0 + x, loc.1 + y);
            }
            1 => {
                dir = dir.turn_right();
                let (x, y) = step(dir);
                loc = (loc.0 + x, loc.1 + y);
            }
            _ => panic!(),
        }
        comp.append_input(*map.entry(loc).or_default());
    }

    let (min_x, min_y, max_x, max_y) = map.keys().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(min_x, min_y, max_x, max_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
        },
    );

    for x in 0..(max_x - min_x + 1) {
        for y in 0..(max_y - min_y + 1) {
            let cell = map
                .get(&(x, y))
                .map(|x| if (*x == 1) { '#' } else { ' ' })
                .unwrap_or(' ');
            print!("{cell}");
        }
        println!();
    }

    0
}
