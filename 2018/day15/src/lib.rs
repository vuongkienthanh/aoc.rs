pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::{FxHashMap, FxHashSet};
use parsing::Item;
use std::collections::VecDeque;
use std::mem;
pub type Map = Vec<Vec<Item>>;
pub type Point = (usize, usize);
type Seen = FxHashSet<Point>;

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

pub enum MoveResult {
    DeadCantMove,
    At(Point),
    NoTarget,
}

fn mov((row, col): Point, map: &mut Map) -> MoveResult {
    match &map[row][col] {
        Item::Space => MoveResult::DeadCantMove,
        Item::Wall => panic!("should not be wall"),
        _ => {
            if is_finish(&*map) {
                return MoveResult::NoTarget;
            }
            // stay
            if adj4((row, col))
                .into_iter()
                .any(|(r, c)| map[r][c].is_enemy_of(&map[row][col]))
            {
                return MoveResult::At((row, col));
            }
            let target_space: FxHashSet<Point> = get_units(&*map)
                .into_iter()
                .filter(|(r, c)| map[*r][*c].is_enemy_of(&map[row][col]))
                .flat_map(|p| adj4(p).into_iter())
                .filter(|(r, c)| map[*r][*c] == Item::Space)
                .collect();
            if target_space.is_empty() {
                return MoveResult::At((row, col));
            }

            let mut visiting = VecDeque::from([((row, col), 0)]);
            let mut meta: FxHashMap<Point, (usize, Option<Point>)> = FxHashMap::default();
            meta.insert((row, col), (0, None));
            let mut seen = FxHashSet::default();

            while let Some(((r, c), step)) = visiting.pop_front() {
                for nb in adj4((r, c))
                    .into_iter()
                    .filter(|(r, c)| matches!(map[*r][*c], Item::Space))
                {
                    meta.entry(nb)
                        .and_modify(|x| *x = (*x).min((step + 1, Some((r, c)))))
                        .or_insert((step + 1, Some((r, c))));
                    if seen.contains(&nb) {
                        continue;
                    }
                    if !visiting.iter().any(|(p, _)| *p == nb) {
                        visiting.push_back((nb, step + 1));
                    }
                }
                seen.insert((r, c));
            }
            if let Some((step, p)) = meta
                .iter()
                .filter(|(p, _)| target_space.contains(p))
                .map(|(p, (step, _))| (step, p))
                .min()
            {
                let mut closest = p.clone();
                while meta.get(&closest).unwrap().0 > 1 {
                    closest = meta.get(&closest).unwrap().1.unwrap();
                }
                let item = mem::take(&mut map[row][col]);
                map[closest.0][closest.1] = item;
                MoveResult::At(closest)
            } else {
                MoveResult::At((row, col))
            }

            // for (dir, p) in adj4((row, col)).into_iter().enumerate() {
            //     let mut seen = Seen::default();
            //     seen.insert((row, col));
            //     let mut current = vec![p];
            //     let mut finish = false;
            //     let mut step = 1;
            //     while !finish && !current.is_empty() {
            //         let mut new: FxHashSet<Point> = FxHashSet::default();
            //         for c in current {
            //             if matches!(map[c.0][c.1], Item::Space) && seen.insert(c) {
            //                 if target_space.contains(&c) {
            //                     target = target.min((step, c.0, c.1, dir));
            //                     finish = true;
            //                 } else {
            //                     new.extend(adj4(c));
            //                 }
            //             }
            //         }
            //         step += 1;
            //         current = new.into_iter().collect();
            //     }
            // }

            // if target == (usize::MAX, usize::MAX, usize::MAX, usize::MAX) {
            //     // println!("{:?} stay {row},{col}", map[row][col]);
            //     MoveResult::At((row, col))
            // } else {
            //     let (new_row, new_col) = adj4((row, col)).get(target.3).cloned().unwrap();
            //     let item = mem::take(&mut map[row][col]);
            //     map[new_row][new_col] = item;
            //     println!(
            //         "{:?} from {row},{col} to {new_row},{new_col}; target={:?},{},{} in {} steps, dir = {}",
            //         map[new_row][new_col],
            //         map[target.1][target.2],
            //         target.1,
            //         target.2,
            //         target.0,
            //         target.3
            //     );
            //     MoveResult::At((new_row, new_col))
            // }
        }
    }
}

fn atk((row, col): Point, map: &mut Map) {
    let current = &map[row][col];
    let mut target = (usize::MAX, usize::MAX, usize::MAX);
    for (r, c) in adj4((row, col)) {
        match map[r][c] {
            Item::Space | Item::Wall => (),
            Item::Goblin(hp) | Item::Elf(hp) => {
                if map[r][c].is_enemy_of(current) {
                    target = target.min((hp, r, c));
                }
            }
        }
    }
    if target != (usize::MAX, usize::MAX, usize::MAX) {
        map[target.1][target.2].got_hit()
    }
}

fn run_once(map: &mut Map) -> bool {
    let mut units = get_units(&*map);
    let mut finish = false;
    for unit in units {
        match mov(unit, map) {
            MoveResult::DeadCantMove => (),
            MoveResult::At((row, col)) => atk((row, col), map),
            MoveResult::NoTarget => {
                finish = true;
                break;
            }
        }
    }
    finish
}

fn is_finish(map: &Map) -> bool {
    let units = get_units(map);
    units.iter().all(|(r, c)| map[*r][*c].is_elf())
        | units.iter().all(|(r, c)| map[*r][*c].is_goblin())
}

fn display_map(map: &Map) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_input;

    #[test]
    fn test_mov() {
        let mut map = parse_input(
            r#"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"#,
        );
        for _ in 0..3 {
            let mut units = get_units(&map);
            units.sort_unstable();
            for unit in units {
                mov(unit, &mut map);
            }
        }

        let result_map = parse_input(
            r#"#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########"#,
        );
        display_map(&map);
        display_map(&result_map);
        assert_eq!(map, result_map);
    }

    #[test]
    fn test_mov2() {
        let mut map = parse_input(
            r#"#######
#.E...#
#.....#
#...G.#
#######"#,
        );
        mov((1, 2), &mut map);
        display_map(&map);
        assert!(matches!(map[1][3], Item::Elf(_)));
    }

    #[test]
    fn test_atk() {
        let mut map = parse_input(
            r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
        );
        for _ in 0..28 {
            run_once(&mut map);
        }
        let mut result_map = parse_input(
            r#"#######
#G....#
#.G...#
#.#.#G#
#...#E#
#....G#
#######"#,
        );
        result_map[2][2] = Item::Goblin(131);
        result_map[3][5] = Item::Goblin(116);
        result_map[4][5] = Item::Elf(113);
        display_map(&map);
        display_map(&result_map);
        assert_eq!(map, result_map);
    }

    #[test]
    fn test_atk_2() {
        let mut map = parse_input(
            r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
        );
        for _ in 0..48 {
            run_once(&mut map);
        }
        let mut result_map = parse_input(
            r#"#######
#G....#
#.G...#
#.#.#G#
#...#.#
#....G#
#######"#,
        );
        result_map[2][2] = Item::Goblin(131);
        result_map[3][5] = Item::Goblin(59);
        assert!(is_finish(&map));
        assert_eq!(map, result_map);
    }
}
