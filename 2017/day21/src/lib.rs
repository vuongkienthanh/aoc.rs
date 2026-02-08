pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::{FxHashMap as Map, FxHashSet as Set};
use parsing::Square;

fn all_transformations(input: Square) -> Set<Square> {
    let mut set = Set::default();

    set.insert(input.clone());
    set.insert(flip_up_down(input.clone()));
    set.insert(flip_left_right(input.clone()));
    set.insert(flip_up_down(flip_left_right(input.clone())));

    let input2 = rotate_left(input);
    set.insert(input2.clone());
    set.insert(flip_up_down(input2.clone()));
    set.insert(flip_left_right(input2.clone()));
    set.insert(flip_up_down(flip_left_right(input2.clone())));

    set
}

fn flip_up_down(input: Square) -> Square {
    input.into_iter().rev().collect()
}
fn flip_left_right(input: Square) -> Square {
    input
        .into_iter()
        .map(|x| x.into_iter().rev().collect())
        .collect()
}

fn rotate_left(input: Square) -> Square {
    let size = input.len();
    let mut new_v = vec![vec!['.'; size]; size];

    for row in 0..size {
        for col in 0..size {
            new_v[col][row] = input[row][col];
        }
    }

    new_v
}

fn build_map(input: Vec<(Square, Square)>) -> Map<Square, Square> {
    let mut map = Map::default();

    for (i, o) in input {
        for t in all_transformations(i) {
            map.insert(t, o.clone());
        }
    }

    map
}
