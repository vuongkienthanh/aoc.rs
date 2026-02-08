pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::FxHashSet;
use parsing::Item;
use std::mem;

pub type Map = Vec<Vec<Item>>;
pub type Point = (usize, usize);

fn adj4((row, col): Point) -> [Point; 4] {
    [
        (row - 1, col),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col),
    ]
}

fn get_units(map: &Map) -> Vec<Point> {
    map.iter()
        .enumerate()
        .flat_map(|(r, line)| line.iter().enumerate().map(move |(c, item)| (r, c, item)))
        .filter_map(|(r, c, item)| (item.is_elf() || item.is_goblin()).then_some((r, c)))
        .collect()
}

/// None if no target
/// else return new location
fn mov((row, col): Point, map: &mut Map) -> Option<Point> {
    match &map[row][col] {
        Item::Space | Item::Wall => panic!("should be a unit"),
        _ => {
            // combat ends
            if map
                .iter()
                .flat_map(|line| line.iter())
                .all(|x| !x.is_enemy_of(&map[row][col]))
            {
                return None;
            }
            // already in range
            if adj4((row, col))
                .into_iter()
                .any(|(r, c)| map[r][c].is_enemy_of(&map[row][col]))
            {
                return Some((row, col));
            }
            // find all ranges
            let target_space: FxHashSet<Point> = map
                .iter()
                .enumerate()
                .flat_map(|(r, line)| line.iter().enumerate().map(move |(c, item)| (r, c, item)))
                .filter_map(|(r, c, item)| item.is_enemy_of(&map[row][col]).then_some((r, c)))
                .flat_map(|p| adj4(p).into_iter())
                .filter(|(r, c)| matches!(map[*r][*c], Item::Space))
                .collect();
            if target_space.is_empty() {
                return Some((row, col));
            }

            // find direction that leads to the nearest range
            let mut current: Vec<Vec<Point>> =
                adj4((row, col)).into_iter().map(|p| vec![p]).collect();
            let mut seen: FxHashSet<Point> = FxHashSet::default();
            let dir = 'a: loop {
                let mut new: Vec<Vec<Point>> = Vec::with_capacity(4);
                for (dir, v) in current.into_iter().enumerate() {
                    let mut new_v = vec![];
                    for (r, c) in v {
                        if matches!(map[r][c], Item::Space) && seen.insert((r, c)) {
                            if target_space.contains(&(r, c)) {
                                break 'a dir;
                            } else {
                                new_v.extend(adj4((r, c)));
                            }
                        }
                    }
                    new_v.sort_unstable();
                    new_v.dedup();
                    new.push(new_v);
                }
                current = new;
                if current.iter().all(|v| v.is_empty()) {
                    return Some((row, col));
                }
            };

            let (new_row, new_col) = match dir {
                0 => (row - 1, col),
                1 => (row, col - 1),
                2 => (row, col + 1),
                3 => (row + 1, col),
                _ => panic!("should be 0..4"),
            };
            let item = mem::take(&mut map[row][col]);
            map[new_row][new_col] = item;
            Some((new_row, new_col))
        }
    }
}

/// return the killed unit
#[derive(Debug)]
enum WhoDead {
    Elf(Point),
    Goblin(Point),
    No,
}
fn atk((row, col): Point, map: &mut Map, atk_power: usize) -> WhoDead {
    let mut target = (usize::MAX, usize::MAX, usize::MAX);
    for (r, c) in adj4((row, col)) {
        match map[r][c] {
            Item::Space | Item::Wall => (),
            Item::Goblin(hp) | Item::Elf(hp) => {
                if map[r][c].is_enemy_of(&map[row][col]) {
                    target = target.min((hp, r, c));
                }
            }
        }
    }
    if target != (usize::MAX, usize::MAX, usize::MAX) {
        let p = (target.1, target.2);
        if map[p.0][p.1].got_hit_and_is_dead(atk_power) {
            if matches!(map[row][col], Item::Goblin(_)) {
                return WhoDead::Elf(p);
            } else {
                return WhoDead::Goblin(p);
            }
        }
    }
    WhoDead::No
}

pub fn display_map(map: &Map) {
    for line in map {
        for c in line {
            match c {
                Item::Space => print!("."),
                Item::Wall => print!("#"),
                Item::Goblin(_) => print!("G"),
                Item::Elf(_) => print!("E"),
            }
        }
        println!();
    }
}
