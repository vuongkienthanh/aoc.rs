pub mod parsing;
pub mod part1;
pub mod part2;

use aoc_helper::direction::Direction;
type Point = (usize, usize);

fn step(p: Point, dir: Direction, input: &[Vec<char>]) -> Option<(Point, Direction)> {
    let new_p = match dir {
        Direction::Up => (p.0 - 1, p.1),
        Direction::Down => (p.0 + 1, p.1),
        Direction::Left => (p.0, p.1 - 1),
        Direction::Right => (p.0, p.1 + 1),
    };
    match input[new_p.0][new_p.1] {
        ' ' => None,
        '+' => Some((
            new_p,
            match dir {
                Direction::Left | Direction::Right => {
                    if [' ', '-'].contains(&input[new_p.0 - 1][new_p.1]) {
                        Direction::Down
                    } else {
                        Direction::Up
                    }
                }
                Direction::Up | Direction::Down => {
                    if [' ', '|'].contains(&input[new_p.0][new_p.1 - 1]) {
                        Direction::Right
                    } else {
                        Direction::Left
                    }
                }
            },
        )),
        _ => Some((new_p, dir)),
    }
}
