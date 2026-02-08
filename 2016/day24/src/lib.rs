pub mod parsing;
pub mod part1;
pub mod part2;

use aoc_helper::adj::naive::adj4;
use grid::Grid;
use parsing::{G, Item, Point};
use std::collections::BTreeMap;

pub fn walk(from: Point, g: &G, number_count: usize) -> BTreeMap<char, usize> {
    let mut ans = BTreeMap::new();
    let mut seen: Grid<bool> = Grid::new(g.rows(), g.cols());

    let mut v: Vec<Point> = vec![];
    v.push(from);
    seen[from] = true;
    let mut step = 0;

    while ans.len() < number_count - 1 {
        let mut new_v = vec![];
        step += 1;
        while let Some(curr) = v.pop() {
            for next in adj4(curr) {
                if seen[next] {
                    continue;
                }
                match g[next] {
                    Item::Wall => continue,
                    Item::Space => {
                        new_v.push(next);
                        seen[next] = true;
                    }
                    Item::Number(d) => {
                        new_v.push(next);
                        seen[next] = true;
                        ans.insert(d, step);
                    }
                }
            }
        }

        v.extend(new_v);
    }

    ans
}
