pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::FxHashSet;
use parsing::Item;
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
    MoveTo(Point),
    NoTarget,
}

fn on_dir(dir: usize, (row, col): Point) -> Point {
    match dir {
        0 => (row - 1, col),
        1 => (row, col - 1),
        2 => (row, col + 1),
        3 => (row + 1, col),
        _ => panic!("should be 0..4"),
    }
}
fn mov((row, col): Point, map: &mut Map) -> MoveResult {
    match &map[row][col] {
        Item::Space => MoveResult::DeadCantMove,
        Item::Wall => panic!("should not be wall"),
        x => {
            if is_finish(&*map) {
                return MoveResult::NoTarget;
            }
            let mut targets = vec![];
            let mut seen = Seen::default();
            seen.insert((row, col));
            let mut current: Vec<Vec<Point>> =
                adj4((row, col)).into_iter().map(|p| vec![p]).collect();
            'a: loop {
                let mut new: Vec<Vec<Point>> = vec![];
                for (dir, v) in current.into_iter().enumerate() {
                    let mut new_v = vec![];
                    for p in v {
                        match &map[p.0][p.1] {
                            &Item::Space => {
                                if seen.insert(p) {
                                    for p2 in adj4(p).into_iter().filter(|x| !seen.contains(x)) {
                                        new_v.push(p2);
                                    }
                                }
                            }
                            &Item::Wall => (),
                            y if !y.is_enemy_of(&x) => (),
                            _ => {
                                targets.push((p, dir));
                            }
                        }
                    }
                    new.push(new_v);
                }
                if !targets.is_empty() {
                    targets.sort_unstable();
                    let ((_, _), dir) = targets.into_iter().next().unwrap();
                    let (new_row, new_col) = on_dir(dir, (row, col));
                    if let Item::Space = map[new_row][new_col] {
                        let item = mem::take(&mut map[row][col]);
                        map[new_row][new_col] = item;
                        break 'a MoveResult::MoveTo((new_row, new_col));
                    } else {
                        break 'a MoveResult::MoveTo((row, col));
                    }
                }
                if new.iter().all(|x| x.is_empty()) {
                    break 'a MoveResult::MoveTo((row, col));
                }

                current = new;
            }
        }
    }
}

fn atk((row, col): Point, map: &mut Map) {
    let current = &map[row][col];
    let mut target = vec![];
    for (r, c) in adj4((row, col)) {
        match map[r][c] {
            Item::Space | Item::Wall => (),
            Item::Goblin(hp) | Item::Elf(hp) => {
                if map[r][c].is_enemy_of(current) {
                    target.push((hp, r, c));
                }
            }
        }
    }
    target.sort_unstable();
    if let Some((_, r, c)) = target.into_iter().next() {
        map[r][c].got_hit()
    }
}

fn run_once(map: &mut Map) -> bool {
    let mut units = get_units(&*map);
    units.sort_unstable();
    for unit in units {
        match mov(unit, map) {
            MoveResult::DeadCantMove => (),
            MoveResult::MoveTo(unit) => atk(unit, map),
            MoveResult::NoTarget => return true,
        }
    }
    false
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
