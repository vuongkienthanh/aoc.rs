use grid::Grid;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
enum CellType {
    Box(Side),
    Empty,
    Wall,
}
#[derive(Debug, Eq, PartialEq, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
enum MoveType {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct MyBox {
    left: Coord,
    right: Coord,
}

type Coord = (usize, usize);

fn parse(input: &str) -> (Coord, Grid<CellType>, Vec<MoveType>) {
    let (map, instruction) = input.split_once("\n\n").unwrap();
    let mut robot = (0, 0);
    let mut expand_map = vec![];
    for (i, line) in map.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.char_indices() {
            match c {
                '.' => {
                    row.push(CellType::Empty);
                    row.push(CellType::Empty);
                }
                'O' => {
                    row.push(CellType::Box(Side::Left));
                    row.push(CellType::Box(Side::Right));
                }
                '@' => {
                    robot = (i, j * 2);
                    row.push(CellType::Empty);
                    row.push(CellType::Empty);
                }
                '#' => {
                    row.push(CellType::Wall);
                    row.push(CellType::Wall);
                }
                _ => (),
            }
        }
        expand_map.push(row);
    }
    let map = Grid::from(expand_map);

    let instruction = instruction
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => MoveType::Up,
                'v' => MoveType::Down,
                '>' => MoveType::Right,
                '<' => MoveType::Left,
                _ => panic!("unknown move type"),
            })
        })
        .collect();
    (robot, map, instruction)
}

pub fn process(_input: &str) -> usize {
    let (mut robot, mut map, instruction) = parse(_input);

    for movetype in instruction {
        step(&mut robot, &mut map, movetype);

        // for (i, row) in map.iter_rows().enumerate() {
        //     for (j, c) in row.enumerate() {
        //         if (i, j) == robot {
        //             print!("@");
        //         } else {
        //             match c {
        //                 CellType::Box(Side::Left) => print!("["),
        //                 CellType::Box(Side::Right) => print!("]"),
        //                 CellType::Empty => print!("."),
        //                 CellType::Wall => print!("#"),
        //             }
        //         }
        //     }
        //     println!();
        // }
    }
    map.indexed_iter()
        .filter(|(_, c)| **c == CellType::Box(Side::Left))
        .map(|((i, j), _)| 100 * i + j)
        .sum()
}
fn step(robot: &mut Coord, map: &mut Grid<CellType>, movetype: MoveType) {
    match movetype {
        MoveType::Up => step_vertical(robot, map, |(x, y)| (x - 1, y)),
        MoveType::Down => step_vertical(robot, map, |(x, y)| (x + 1, y)),
        MoveType::Left => step_horizontal(robot, map, |(x, y)| (x, y - 1)),
        MoveType::Right => step_horizontal(robot, map, |(x, y)| (x, y + 1)),
    }
}
fn step_vertical(robot: &mut Coord, map: &mut Grid<CellType>, dir: impl Fn(Coord) -> Coord) {
    let candidate = dir(*robot);
    match &map[candidate] {
        CellType::Box(side) => {
            let box_dir = |b: MyBox| MyBox {
                left: dir(b.left),
                right: dir(b.right),
            };
            let candidate_box = match side {
                Side::Left => MyBox {
                    left: candidate,
                    right: (candidate.0, candidate.1 + 1),
                },
                Side::Right => MyBox {
                    left: (candidate.0, candidate.1 - 1),
                    right: candidate,
                },
            };
            let mut to_push = HashSet::new();
            if try_push_vertical(HashSet::from([candidate_box]), map, box_dir, &mut to_push) {
                for box_ in to_push.iter() {
                    map[box_.left] = CellType::Empty;
                    map[box_.right] = CellType::Empty;
                }
                for box_ in to_push {
                    let new_box_ = box_dir(box_);
                    map[new_box_.left] = CellType::Box(Side::Left);
                    map[new_box_.right] = CellType::Box(Side::Right);
                }
                *robot = candidate;
            }
        }
        CellType::Empty => {
            *robot = candidate;
        }
        CellType::Wall => (),
    }
}
fn try_push_vertical(
    boxes: HashSet<MyBox>,
    map: &mut Grid<CellType>,
    box_dir: impl Fn(MyBox) -> MyBox,
    to_push: &mut HashSet<MyBox>,
) -> bool {
    let mut new_boxes = HashSet::new();
    for box_ in boxes {
        to_push.insert(box_.clone());
        let candidate_box = box_dir(box_);
        if map[candidate_box.left] == CellType::Wall || map[candidate_box.right] == CellType::Wall {
            return false;
        }
        if let CellType::Box(Side::Left) = map[candidate_box.left] {
            new_boxes.insert(candidate_box.clone());
        }
        if let CellType::Box(Side::Right) = map[candidate_box.left] {
            new_boxes.insert(MyBox {
                left: (candidate_box.left.0, candidate_box.left.1 - 1),
                right: candidate_box.left,
            });
        }
        if let CellType::Box(Side::Left) = map[candidate_box.right] {
            new_boxes.insert(MyBox {
                left: candidate_box.right,
                right: (candidate_box.right.0, candidate_box.right.1 + 1),
            });
        }
    }
    if new_boxes.is_empty() {
        true
    } else {
        try_push_vertical(new_boxes, map, box_dir, to_push)
    }
}

fn step_horizontal(robot: &mut Coord, map: &mut Grid<CellType>, dir: impl Fn(Coord) -> Coord) {
    let candidate = dir(*robot);
    match &map[candidate] {
        CellType::Box(_) => {
            if try_push_horizontal(candidate, dir(candidate), map, dir) {
                *robot = candidate;
            }
        }
        CellType::Empty => *robot = candidate,
        CellType::Wall => (),
    }
}
fn try_push_horizontal(
    prv: Coord,
    nxt: Coord,
    map: &mut Grid<CellType>,
    dir: impl Fn(Coord) -> Coord,
) -> bool {
    let candidate = dir(nxt);
    match map[candidate] {
        CellType::Box(_) => {
            if try_push_horizontal(candidate, dir(candidate), map, dir) {
                map[candidate] = map[nxt].clone();
                map[nxt] = map[prv].clone();
                map[prv] = CellType::Empty;
                true
            } else {
                false
            }
        }
        CellType::Empty => {
            map[candidate] = map[nxt].clone();
            map[nxt] = map[prv].clone();
            map[prv] = CellType::Empty;
            true
        }
        CellType::Wall => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // #[ignore]
    fn test_process() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        assert_eq!(process(input), 9021);
    }
    #[test]
    #[ignore]
    fn test_process2() {
        let input = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;
        assert_eq!(process(input), 105);
    }
}
