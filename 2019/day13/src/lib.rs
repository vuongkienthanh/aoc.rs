pub mod part1;
pub mod part2;
use std::collections::BTreeMap;

fn screen(map: &BTreeMap<(usize, usize), Tile>) {
    let (max_x, max_y) = map.keys().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    });
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!(
                "{}",
                match map.get(&(x, y)).unwrap() {
                    Tile::empty => '.',
                    Tile::wall => '#',
                    Tile::block => 'x',
                    Tile::paddle => '-',
                    Tile::ball => 'o',
                }
            );
        }
        println!();
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Tile {
    empty,
    wall,
    block,
    paddle,
    ball,
}
impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Tile::empty,
            1 => Tile::wall,
            2 => Tile::block,
            3 => Tile::paddle,
            4 => Tile::ball,
            _ => panic!("should be 0 1 2 3 4"),
        }
    }
}
impl Tile {
    pub fn is_indestructible(&self) -> bool {
        matches!(self, Tile::wall | Tile::paddle)
    }
}
