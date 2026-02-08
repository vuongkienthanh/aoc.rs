pub mod parsing;
pub mod part1;
pub mod part2;

use parsing::{Carts, Direction, Map, Path};

fn step(carts: &mut Carts, map: &Map) -> Vec<(usize, usize)> {
    let mut cart_loc: Vec<_> = carts.keys().cloned().collect();
    cart_loc.sort_unstable();

    let mut crashes: Vec<(usize, usize)> = Vec::new();

    for (mut row, mut col) in cart_loc {
        if let Some((mut dir, mut turn)) = carts.remove(&(row, col)) {
            match dir {
                Direction::Up => row -= 1,
                Direction::Down => row += 1,
                Direction::Left => col -= 1,
                Direction::Right => col += 1,
            }
            if let Some(_) = carts.remove(&(row, col)) {
                crashes.push((row, col));
                continue;
            }
            match map[row][col] {
                Path::Straight => (),
                Path::Cross => match turn {
                    0 => {
                        dir = dir.turn_left();
                        turn = 1;
                    }
                    1 => {
                        turn = 2;
                    }
                    2 => {
                        dir = dir.turn_right();
                        turn = 0;
                    }
                    _ => panic!("should be 0 1 2"),
                },
                Path::Forward => match dir {
                    Direction::Up | Direction::Down => dir = dir.turn_right(),
                    Direction::Left | Direction::Right => dir = dir.turn_left(),
                },
                Path::Backward => match dir {
                    Direction::Up | Direction::Down => dir = dir.turn_left(),
                    Direction::Left | Direction::Right => dir = dir.turn_right(),
                },
                Path::Blank => panic!("should not on blank"),
            }
            carts.insert((row, col), (dir, turn));
        }
    }
    crashes
}
