use crate::build_map;
use crate::parsing::{Square, parse_input};

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let map = build_map(input);
    let mut grid: Square = r#".#.
..#
###"#
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    for _ in 0..5 {
        let size = grid.len();
        let mut new_grid = vec![];
        if size.is_multiple_of(2) {
            for row_chunk in grid.chunks(2) {
                let mut new_chunk = vec![vec![]; 3];
                for (a, b) in row_chunk[0].chunks(2).zip(row_chunk[1].chunks(2)) {
                    let square = vec![a.to_vec(), b.to_vec()];
                    for (i, row) in map.get(&square).cloned().unwrap().into_iter().enumerate() {
                        new_chunk[i].extend(row)
                    }
                }
                for row in new_chunk {
                    new_grid.push(row)
                }
            }
        } else if size.is_multiple_of(3) {
            for row_chunk in grid.chunks(3) {
                let mut new_chunk = vec![vec![]; 4];
                for ((a, b), c) in row_chunk[0]
                    .chunks(3)
                    .zip(row_chunk[1].chunks(3))
                    .zip(row_chunk[2].chunks(3))
                {
                    let square = vec![a.to_vec(), b.to_vec(), c.to_vec()];
                    for (i, row) in map.get(&square).cloned().unwrap().into_iter().enumerate() {
                        new_chunk[i].extend(row)
                    }
                }
                for row in new_chunk {
                    new_grid.push(row)
                }
            }
        }
        grid = new_grid;
    }

    grid.into_iter()
        .map(|line| line.into_iter().filter(|x| *x == '#').count())
        .sum()
}
