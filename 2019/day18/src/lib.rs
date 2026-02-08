pub mod part1;
pub mod part2;

use aoc_helper::adj::naive::adj4;
use std::collections::{BTreeMap as Map, BTreeSet as Set};

pub type Point = (usize, usize);

fn parse(input: &str) -> (Vec<Vec<u8>>, Point, Vec<Point>) {
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut keys = vec![];
    for (row, line) in input.lines().enumerate() {
        let mut new_row = vec![];
        for (col, cell) in line.bytes().enumerate() {
            match cell {
                b'#' | b'.' | b'A'..=b'Z' => new_row.push(cell),
                b'@' => {
                    new_row.push(b'.');
                    start = (row, col);
                }
                b'a'..=b'z' => {
                    new_row.push(cell);
                    keys.push((row, col));
                }
                _ => panic!(),
            }
        }
        grid.push(new_row);
    }
    (grid, start, keys)
}

fn graph_bfs(grid: &[Vec<u8>], key: Point) -> Vec<(Point, u32, usize)> {
    let mut ans = vec![];
    let mut step = 0;
    let mut current = vec![(0, key)];
    let mut seen = Set::new();
    seen.insert(key);

    while !current.is_empty() {
        step += 1;
        let mut new = vec![];
        for (doors, loc) in current {
            for (r, c) in adj4(loc) {
                if seen.insert((r, c)) {
                    match grid[r][c] {
                        b'#' => (),
                        b'.' => new.push((doors, (r, c))),
                        b'a'..=b'z' => {
                            new.push((doors, (r, c)));
                            ans.push(((r, c), doors, step));
                        }
                        b'A'..=b'Z' => {
                            let mut doors = doors;
                            let key = 1 << (grid[r][c] + 32 - b'a') as u32;
                            doors |= key;
                            new.push((doors, (r, c)));
                        }
                        _ => panic!(),
                    }
                }
            }
        }

        current = new;
    }

    ans
}
fn build_graph(
    keys: &[Point],
    starts: &[Point],
    grid: &[Vec<u8>],
) -> Map<Point, Vec<(Point, u32, usize)>> {
    keys.iter()
        .chain(starts)
        .cloned()
        .map(|p| (p, graph_bfs(grid, p)))
        .collect()
}
