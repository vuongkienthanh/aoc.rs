pub mod parsing;
pub mod part1;
pub mod part2;

use aoc_helper::direction::Direction;

fn forward(direction: Direction, c: (isize, isize), n: isize) -> (isize, isize) {
    use Direction::*;
    match direction {
        Up => (c.0 - n, c.1),
        Down => (c.0 + n, c.1),
        Left => (c.0, c.1 - n),
        Right => (c.0, c.1 + n),
    }
}
