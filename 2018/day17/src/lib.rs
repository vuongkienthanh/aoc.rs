pub mod parsing;
pub mod part1;
pub mod part2;
use aoc_helper::range::merge;
use parsing::Item;
use std::collections::BTreeMap;

pub type Map = BTreeMap<usize, Vec<(usize, usize)>>;
pub type Point = (usize, usize);

fn build_bases_and_walls(input: Vec<Item>) -> (Map, Map, usize) {
    let (mut bases, mut walls, max_y) = input.into_iter().fold(
        (Map::new(), Map::new(), usize::MIN),
        |(mut bases, mut walls, mut max_y), item| {
            match item {
                Item::X(x, r) => {
                    max_y = max_y.max(r.1);
                    walls.entry(x).or_default().push(r);
                    bases.entry(r.0).or_default().push((x, x));
                }
                Item::Y(y, r) => {
                    max_y = max_y.max(y);
                    bases.entry(y).or_default().push(r);
                    walls.entry(r.0).or_default().push((y, y));
                    walls.entry(r.1).or_default().push((y, y));
                }
            };
            (bases, walls, max_y)
        },
    );
    bases = bases.into_iter().map(|(k, v)| (k, merge(v))).collect();
    walls = walls.into_iter().map(|(k, v)| (k, merge(v))).collect();
    (bases, walls, max_y)
}

fn find_base(spring: Point, bases: &Map) -> Option<(usize, (usize, usize))> {
    bases.iter().find_map(|(y, v)| {
        if *y > spring.1 {
            v.iter()
                .find_map(|r| (r.0 <= spring.0 && r.1 >= spring.0).then_some((*y, (r.0, r.1))))
        } else {
            None
        }
    })
}

fn find_wall(
    spring: Point,
    y: usize,
    (left, right): (usize, usize),
    walls: &Map,
) -> (Option<usize>, Option<usize>) {
    let mut x: Vec<usize> = walls
        .keys()
        .filter(|x| **x >= left && **x <= right)
        .cloned()
        .collect();
    let mut left_x: Vec<usize> = x.extract_if(.., |x| *x < spring.0).collect();
    left_x.reverse();
    let right_x = x;
    (
        left_x
            .into_iter()
            .find(|x| walls.get(x).unwrap().iter().any(|v| v.0 <= y && v.1 >= y)),
        right_x
            .into_iter()
            .find(|x| walls.get(x).unwrap().iter().any(|v| v.0 <= y && v.1 >= y)),
    )
}

fn display_map(bases: &Map, walls: &Map) {
    let max_y = bases
        .keys()
        .cloned()
        .chain(walls.values().map(|v| v.iter().map(|r| r.1).max().unwrap()))
        .max()
        .unwrap();
    let min_x = walls
        .keys()
        .cloned()
        .chain(bases.values().map(|v| v.iter().map(|r| r.0).min().unwrap()))
        .min()
        .unwrap();
    let max_x = walls
        .keys()
        .cloned()
        .chain(bases.values().map(|v| v.iter().map(|r| r.1).max().unwrap()))
        .max()
        .unwrap();

    let mut map = vec![vec!['.'; max_x - min_x + 1]; max_y + 1];
    for (y, v) in bases {
        for r in v {
            for x in r.0..=r.1 {
                map[*y][x - min_x] = '#';
            }
        }
    }
    for (x, v) in walls {
        for r in v {
            for y in r.0..=r.1 {
                map[y][x - min_x] = '#';
            }
        }
    }

    for row in map {
        for cell in row {
            print!("{cell}");
        }
        println!("")
    }
}
