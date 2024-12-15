use grid::Grid;

#[derive(Debug, Eq, PartialEq)]
enum CellType {
    Box,
    Empty,
    Wall,
}

#[derive(Debug)]
enum MoveType {
    Up,
    Down,
    Left,
    Right,
}

type Coord = (usize, usize);

fn parse(input: &str) -> (Coord, Grid<CellType>, Vec<MoveType>) {
    let (map, instruction) = input.split_once("\n\n").unwrap();
    let mut robot = (0, 0);
    let map = Grid::from(
        map.lines()
            .enumerate()
            .map(|(i, line)| {
                line.char_indices()
                    .map(|(j, c)| match c {
                        '.' => CellType::Empty,
                        '#' => CellType::Wall,
                        'O' => CellType::Box,
                        '@' => {
                            robot = (i, j);
                            CellType::Empty
                        }
                        _ => panic!("unknown cell type"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
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
    }
    map.indexed_iter()
        .filter(|(_, c)| **c == CellType::Box)
        .map(|((i, j), _)| 100 * i + j)
        .sum()
}

fn step(robot: &mut Coord, map: &mut Grid<CellType>, movetype: MoveType) {
    let dir = match movetype {
        MoveType::Up => |(x, y)| (x - 1, y),
        MoveType::Down => |(x, y)| (x + 1, y),
        MoveType::Left => |(x, y)| (x, y - 1),
        MoveType::Right => |(x, y)| (x, y + 1),
    };
    let candidate = dir(*robot);
    match map[candidate] {
        CellType::Box => {
            if push(candidate, map, dir) {
                *robot = candidate;
            }
        }
        CellType::Empty => {
            *robot = candidate;
        }
        CellType::Wall => (),
    }
}
fn push(box_: Coord, map: &mut Grid<CellType>, dir: impl Fn(Coord) -> Coord) -> bool {
    let candidate = dir(box_);
    match map[candidate] {
        CellType::Box => {
            if push(candidate, map, dir) {
                map[candidate] = CellType::Box;
                map[box_] = CellType::Empty;
                true
            } else {
                false
            }
        }
        CellType::Empty => {
            map[candidate] = CellType::Box;
            map[box_] = CellType::Empty;
            true
        }
        CellType::Wall => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
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
        assert_eq!(process(input), 10092);
    }
    #[test]
    fn test_process2() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        assert_eq!(process(input), 2028);
    }
}
