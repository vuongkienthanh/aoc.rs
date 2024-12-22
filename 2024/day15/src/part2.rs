use crate::{parse_instruction, print_grid, CellType, Coord, MoveType, MyBox, Side};
use grid::Grid;
use std::collections::HashSet;

fn parse(input: &str) -> (Coord, Grid<CellType>, Vec<MoveType>) {
    let (grid, instruction) = input.split_once("\n\n").unwrap();
    let mut robot = (0, 0);
    let mut expand_grid = vec![];
    for (i, line) in grid.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.char_indices() {
            match c {
                '.' => {
                    row.push(CellType::Empty);
                    row.push(CellType::Empty);
                }
                'O' => {
                    row.push(CellType::DBox(Side::Left));
                    row.push(CellType::DBox(Side::Right));
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
        expand_grid.push(row);
    }
    let grid = Grid::from(expand_grid);
    let instruction = parse_instruction(instruction);
    (robot, grid, instruction)
}

pub fn process(_input: &str) -> usize {
    let (mut robot, mut grid, instruction) = parse(_input);

    for movetype in instruction {
        step(&mut robot, &mut grid, movetype);
        print_grid(robot, &grid);
    }
    grid.indexed_iter()
        .filter(|(_, c)| **c == CellType::DBox(Side::Left))
        .map(|((i, j), _)| 100 * i + j)
        .sum()
}
fn step(robot: &mut Coord, grid: &mut Grid<CellType>, movetype: MoveType) {
    match movetype {
        MoveType::Up => step_vertical(robot, grid, |(x, y)| (x - 1, y)),
        MoveType::Down => step_vertical(robot, grid, |(x, y)| (x + 1, y)),
        MoveType::Left => step_horizontal(robot, grid, |(x, y)| (x, y - 1)),
        MoveType::Right => step_horizontal(robot, grid, |(x, y)| (x, y + 1)),
    }
}
fn step_vertical(robot: &mut Coord, grid: &mut Grid<CellType>, dir: impl Fn(Coord) -> Coord) {
    let candidate = dir(*robot);
    match &grid[candidate] {
        CellType::DBox(side) => {
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
            let box_dir = |b: &MyBox| MyBox {
                left: dir(b.left),
                right: dir(b.right),
            };
            if try_push_vertical(HashSet::from([candidate_box]), grid, box_dir) {
                *robot = candidate;
            }
        }
        CellType::Empty => {
            *robot = candidate;
        }
        _ => (),
    }
}
fn try_push_vertical(
    boxes: HashSet<MyBox>,
    grid: &mut Grid<CellType>,
    box_dir: impl Fn(&MyBox) -> MyBox,
) -> bool {
    let mut candidates = vec![];
    let mut new_boxes = HashSet::new();
    for box_ in &boxes {
        let candidate = box_dir(box_);
        if grid[candidate.left] == CellType::Wall || grid[candidate.right] == CellType::Wall {
            return false;
        }
        candidates.push(candidate.clone());
        if let CellType::DBox(Side::Left) = grid[candidate.left] {
            new_boxes.insert(candidate.clone());
        }
        if let CellType::DBox(Side::Right) = grid[candidate.left] {
            new_boxes.insert(MyBox {
                left: (candidate.left.0, candidate.left.1 - 1),
                right: candidate.left,
            });
        }
        if let CellType::DBox(Side::Left) = grid[candidate.right] {
            new_boxes.insert(MyBox {
                left: candidate.right,
                right: (candidate.right.0, candidate.right.1 + 1),
            });
        }
    }
    if new_boxes.is_empty() || try_push_vertical(new_boxes, grid, box_dir) {
        for box_ in boxes {
            grid[box_.left] = CellType::Empty;
            grid[box_.right] = CellType::Empty;
        }
        for box_ in candidates {
            grid[box_.left] = CellType::DBox(Side::Left);
            grid[box_.right] = CellType::DBox(Side::Right);
        }
        true
    } else {
        false
    }
}

fn step_horizontal(robot: &mut Coord, grid: &mut Grid<CellType>, dir: impl Fn(Coord) -> Coord) {
    let candidate = dir(*robot);
    match &grid[candidate] {
        CellType::DBox(_) => {
            if try_push_horizontal(candidate, grid, dir) {
                *robot = candidate;
            }
        }
        CellType::Empty => *robot = candidate,
        _ => (),
    }
}
fn try_push_horizontal(
    box_tail: Coord,
    grid: &mut Grid<CellType>,
    dir: impl Fn(Coord) -> Coord,
) -> bool {
    let box_head = dir(box_tail);
    let candidate = dir(box_head);
    match grid[candidate] {
        CellType::DBox(_) => {
            if try_push_horizontal(candidate, grid, dir) {
                grid[candidate] = grid[box_head].clone();
                grid[box_head] = grid[box_tail].clone();
                grid[box_tail] = CellType::Empty;
                true
            } else {
                false
            }
        }
        CellType::Empty => {
            grid[candidate] = grid[box_head].clone();
            grid[box_head] = grid[box_tail].clone();
            grid[box_tail] = CellType::Empty;
            true
        }
        _ => false,
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
    // #[ignore]
    fn test_process2() {
        let input = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;
        assert_eq!(process(input), 618);
    }
}
