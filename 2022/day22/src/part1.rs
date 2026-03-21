use crate::parsing::{Cell, Op, parse_input};
use aoc_helper::direction::Direction;

pub fn process(_input: &str) -> usize {
    let (map, ops) = parse_input(_input);
    let mut loc: (usize, usize, Direction) = map
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(i, c)| matches!(c, Cell::Space).then_some((0, i, Direction::Right)))
        .unwrap();
    for op in ops {
        match op {
            Op::L => loc.2 = loc.2.turn_left(),
            Op::R => loc.2 = loc.2.turn_right(),
            Op::Mv(i) => match loc.2 {
                Direction::Up => {
                    let mut target_r = loc.0;
                    for _ in 0..i {
                        let new_r = (target_r + map.len() - 1) % map.len();
                        match map
                            .get(new_r)
                            .unwrap()
                            .get(loc.1)
                            .cloned()
                            .unwrap_or_default()
                        {
                            Cell::Empty => {
                                let (cell, new_r) = map[loc.0..]
                                    .iter()
                                    .zip(loc.0..)
                                    .map(|(row, r)| {
                                        (row.get(loc.1).cloned().unwrap_or_default(), r)
                                    })
                                    .take_while(|(cell, _)| !matches!(cell, Cell::Empty))
                                    .last()
                                    .unwrap();
                                match cell {
                                    Cell::Empty => panic!(),
                                    Cell::Space => target_r = new_r,
                                    Cell::Wall => {
                                        loc.0 = target_r;
                                        break;
                                    }
                                }
                            }
                            Cell::Space => target_r = new_r,
                            Cell::Wall => {
                                loc.0 = target_r;
                                break;
                            }
                        }
                    }
                    loc.0 = target_r;
                }
                Direction::Down => {
                    let mut target_r = loc.0;
                    for _ in 0..i {
                        let new_r = (target_r + 1) % map.len();
                        match map
                            .get(new_r)
                            .unwrap()
                            .get(loc.1)
                            .cloned()
                            .unwrap_or_default()
                        {
                            Cell::Empty => {
                                let (cell, new_r) = map[0..=loc.0]
                                    .iter()
                                    .rev()
                                    .zip((0..=loc.0).rev())
                                    .map(|(row, r)| {
                                        (row.get(loc.1).cloned().unwrap_or_default(), r)
                                    })
                                    .take_while(|(cell, _)| !matches!(cell, Cell::Empty))
                                    .last()
                                    .unwrap();
                                match cell {
                                    Cell::Empty => panic!(),
                                    Cell::Space => target_r = new_r,
                                    Cell::Wall => {
                                        loc.0 = target_r;
                                        break;
                                    }
                                }
                            }
                            Cell::Space => target_r = new_r,
                            Cell::Wall => {
                                loc.0 = target_r;
                                break;
                            }
                        }
                    }
                    loc.0 = target_r;
                }
                Direction::Left => {
                    let mut target_c = loc.1;
                    let row = map.get(loc.0).unwrap();
                    let len = row.len();
                    for _ in 0..i {
                        let new_c = (target_c + len - 1) % len;
                        match row.get(new_c).unwrap() {
                            Cell::Empty => {
                                let (cell, new_c) = row[loc.1..]
                                    .iter()
                                    .zip(loc.1..)
                                    .take_while(|(cell, _)| !matches!(cell, Cell::Empty))
                                    .last()
                                    .unwrap();
                                match cell {
                                    Cell::Empty => panic!(),
                                    Cell::Space => target_c = new_c,
                                    Cell::Wall => {
                                        loc.1 = target_c;
                                        break;
                                    }
                                }
                            }
                            Cell::Space => target_c = new_c,
                            Cell::Wall => {
                                loc.1 = target_c;
                                break;
                            }
                        }
                    }
                    loc.1 = target_c;
                }
                Direction::Right => {
                    let mut target_c = loc.1;
                    let row = map.get(loc.0).unwrap();
                    let len = row.len();
                    for _ in 0..i {
                        let new_c = (target_c + 1) % len;
                        match row.get(new_c).unwrap() {
                            Cell::Empty => {
                                let (cell, new_c) = row[0..=loc.1]
                                    .iter()
                                    .rev()
                                    .zip((0..=loc.1).rev())
                                    .take_while(|(cell, _)| !matches!(cell, Cell::Empty))
                                    .last()
                                    .unwrap();
                                match cell {
                                    Cell::Empty => panic!(),
                                    Cell::Space => target_c = new_c,
                                    Cell::Wall => {
                                        loc.1 = target_c;
                                        break;
                                    }
                                }
                            }
                            Cell::Space => target_c = new_c,
                            Cell::Wall => {
                                loc.1 = target_c;
                                break;
                            }
                        }
                    }
                    loc.1 = target_c;
                }
            },
        }
    }

    (loc.0 + 1) * 1000
        + (loc.1 + 1) * 4
        + match loc.2 {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#
    }

    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 6032);
    }
}
