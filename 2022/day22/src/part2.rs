use crate::parsing::{Cell, Op, parse_input};
use crate::{Border, FACES, build_cube};
use aoc_helper::direction::Direction;
use grid::Grid;

fn run(
    mut loc: (usize, usize, Direction, usize),
    i: usize,
    cube: &[Grid<Cell>],
) -> (usize, usize, Direction, usize) {
    for _ in 0..i {
        let mut new_loc = loc.clone();
        match loc.2 {
            Direction::Up => match loc.0 {
                0 => {
                    let (face, border) = &FACES[loc.3][0];
                    new_loc.3 = *face;
                    match border {
                        Border::Up => {
                            new_loc.0 = 0;
                            new_loc.1 = 49 - loc.1;
                            new_loc.2 = Direction::Down;
                        }
                        Border::Down => {
                            new_loc.0 = 49;
                            new_loc.1 = loc.1;
                            new_loc.2 = Direction::Up;
                        }
                        Border::Left => {
                            new_loc.0 = loc.1;
                            new_loc.1 = 0;
                            new_loc.2 = Direction::Right;
                        }
                        Border::Right => {
                            new_loc.0 = 49 - loc.1;
                            new_loc.1 = 49;
                            new_loc.2 = Direction::Left;
                        }
                    }
                }
                _ => new_loc.0 = loc.0 - 1,
            },
            Direction::Down => match loc.0 {
                49 => {
                    let (face, border) = &FACES[loc.3][1];
                    new_loc.3 = *face;
                    match border {
                        Border::Up => {
                            new_loc.0 = 0;
                            new_loc.1 = loc.1;
                            new_loc.2 = Direction::Down;
                        }
                        Border::Down => {
                            new_loc.0 = 49;
                            new_loc.1 = 49 - loc.1;
                            new_loc.2 = Direction::Up;
                        }
                        Border::Left => {
                            new_loc.0 = 49 - loc.1;
                            new_loc.1 = 0;
                            new_loc.2 = Direction::Right;
                        }
                        Border::Right => {
                            new_loc.0 = loc.1;
                            new_loc.1 = 49;
                            new_loc.2 = Direction::Left;
                        }
                    }
                }
                _ => new_loc.0 = loc.0 + 1,
            },
            Direction::Left => match loc.1 {
                0 => {
                    let (face, border) = &FACES[loc.3][2];
                    new_loc.3 = *face;
                    match border {
                        Border::Up => {
                            new_loc.0 = 0;
                            new_loc.1 = loc.0;
                            new_loc.2 = Direction::Down;
                        }
                        Border::Down => {
                            new_loc.0 = 49;
                            new_loc.1 = 49 - loc.0;
                            new_loc.2 = Direction::Up;
                        }
                        Border::Left => {
                            new_loc.0 = 49 - loc.0;
                            new_loc.1 = 0;
                            new_loc.2 = Direction::Right;
                        }
                        Border::Right => {
                            new_loc.0 = loc.0;
                            new_loc.1 = 49;
                            new_loc.2 = Direction::Left;
                        }
                    }
                }
                _ => new_loc.1 = loc.1 - 1,
            },
            Direction::Right => match loc.1 {
                49 => {
                    let (face, border) = &FACES[loc.3][3];
                    new_loc.3 = *face;
                    match border {
                        Border::Up => {
                            new_loc.0 = 0;
                            new_loc.1 = 49 - loc.0;
                            new_loc.2 = Direction::Down;
                        }
                        Border::Down => {
                            new_loc.0 = 49;
                            new_loc.1 = loc.0;
                            new_loc.2 = Direction::Up;
                        }
                        Border::Left => {
                            new_loc.0 = loc.0;
                            new_loc.1 = 0;
                            new_loc.2 = Direction::Right;
                        }
                        Border::Right => {
                            new_loc.0 = 49 - loc.0;
                            new_loc.1 = 49;
                            new_loc.2 = Direction::Left;
                        }
                    }
                }
                _ => new_loc.1 = loc.1 + 1,
            },
        }
        if matches!(cube[new_loc.3][(new_loc.0, new_loc.1)], Cell::Wall) {
            break;
        } else {
            loc = new_loc;
        }
    }
    loc
}

pub fn process(_input: &str) -> usize {
    let (map, ops) = parse_input(_input);
    let cube = build_cube(map);
    let mut loc = (0, 0, Direction::Right, 0);

    for op in ops {
        match op {
            Op::L => loc.2 = loc.2.turn_left(),
            Op::R => loc.2 = loc.2.turn_right(),
            Op::Mv(i) => loc = run(loc.clone(), i, &cube),
        }
    }
    match loc.3 {
        0 => {
            loc.1 = loc.1 + 50;
        }
        1 => {
            loc.1 = loc.1 + 100;
        }
        2 => {
            loc.0 = loc.0 + 50;
            loc.1 = loc.1 + 50;
        }
        3 => {
            loc.0 = loc.0 + 100;
        }
        4 => {
            loc.0 = loc.0 + 100;
            loc.1 = loc.1 + 50;
        }
        5 => {
            loc.0 = loc.0 + 150;
        }
        _ => panic!(),
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
