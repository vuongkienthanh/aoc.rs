use super::{parse, CellType, Coord, Direction, Guard };
use grid::Grid;
use std::collections::{HashMap, HashSet};

pub fn process(_input: &str) -> usize {
    let (mut guard, grid) = parse(_input);
    let origin_guard = guard.clone();
    let mut visited = Grid::new(grid.rows(), grid.cols());
    visited.fill(0);


    todo!()

    // visited
    //     .into_iter()
    //     .filter(|obs| {
    //         // prepare
    //         let mut grid = grid.clone();
    //         let mut guard = origin_guard.clone();
    //         *grid.get_mut(obs[0], obs[1]).unwrap() = CellType::Obstacle;
    //         let mut seen = HashMap::from([
    //             (Direction::North, false),
    //             (Direction::South, false),
    //             (Direction::East, false),
    //             (Direction::West, false),
    //         ]);
    //         // run
    //         let mut is_loop = false;
    //         while let Some((move_type, next_guard)) = guard.try_forward(&grid) {
    //             guard = next_guard;
    //             // check loop
    //             if let MoveType::Turn {
    //                 guard_direction_when_approach_obstacle: direction,
    //                 obstacle_position: position,
    //             } = move_type
    //             {
    //                 if &position == obs {
    //                     if *seen.get(&direction).unwrap() {
    //                         is_loop = true;
    //                         break;
    //                     } else {
    //                         *seen.get_mut(&direction).unwrap() = true
    //                     }
    //                 }
    //             }
    //         }
    //
    //         is_loop
    //     })
    //     .count()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(process(input), 6);
    }
}
