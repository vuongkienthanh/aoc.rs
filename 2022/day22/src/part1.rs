use crate::parsing::parse_input;
use aoc_helper::direction::Direction;

pub fn process(_input: &str) -> usize {
    let (map, ops) = parse_input(_input);
    let mut start: (usize, usize, Direction) = map
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(i, c)| matches!(c, Cell::Space).then_some((0, i, Direction::Right)))
        .unwrap();
    // for line in &map {
    //     for cell in line {
    //         match cell {
    //             Cell::Empty => print!(" "),
    //             Cell::Wall => print!("#"),
    //             Cell::Space => print!("."),
    //         }
    //     }
    //     println!();
    // }
    // println!("{ops:?}");
    for op in ops {
        match op {
            Op::L => start.2 = start.2.turn_left(),
            Op::R => start.2 = start.2.turn_right(),
            Op::Mv(i) => match start.2 {
                Direction::Up => {
                    for _ in 0..i {
                        let target_r = (start.0 + map.len() - 1) % map.len();
                        match map.get(target_r).unwrap().get(start.1).cloned().unwrap_or_default() {
                            Cell::Empty => {
                                let (target_r, cell) = map
                                    .iter()
                                    .enumerate()
                                    .skip(start.0)
                                    .map(|(r, row)| {
                                        (r, row.get(start.1).cloned().unwrap_or_default())
                                    })
                                    .take_while(|(_, cell)| !matches!(cell, Cell::Empty))
                                    .last()
                                    .unwrap();
                                match cell {
                                    Cell::Empty => panic!(),
                                    Cell::Space => start.0 = target_r,
                                    Cell::Wall => break,
                                }
                            }
                            Cell::Space => start.0 = target_r,
                            Cell::Wall => break,
                        }
                    }
                }
                Direction::Down => {}
                Direction::Left => {
                    for _ in 0..i {
                        let len = map.get(start.0).unwrap().len();
                        let target_c = (start.1 + len - 1) % len;
                        match map.get(start.0).unwrap().get(target_c).unwrap() {
                            Cell::Empty => {
                                let (target_c, cell) = map
                                    .get(start.0)
                                    .unwrap()
                                    .iter()
                                    .enumerate()
                                    .skip(start.1)
                                    .take_while(|(_, cell)| !matches!(cell, Cell::Empty))
                                    .last()
                                    .unwrap();
                                match cell {
                                    Cell::Empty => panic!(),
                                    Cell::Space => start.1 = target_c,
                                    Cell::Wall => break,
                                }
                            }
                            Cell::Space => start.1 = target_c,
                            Cell::Wall => break,
                        }
                    }
                }
                Direction::Right => {}
            },
        }
    }

    (start.0 + 1) * 1000
        + (start.1 + 1) * 4
        + match start.2 {
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
