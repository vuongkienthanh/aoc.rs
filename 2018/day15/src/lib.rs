pub mod parsing;
pub mod part1;
pub mod part2;

use fxhash::FxHashSet;
use parsing::Item;
pub type Map = Vec<Vec<Item>>;
pub type Point = (usize, usize);

fn adj4((row, col): Point) -> [Point; 4] {
    [
        (row - 1, col),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col),
    ]
}

fn sort(units: &mut Vec<(Point, usize)>) {
    units.sort_unstable_by(|a, b| b.0.cmp(&a.0))
}

fn mov((row, col): Point, units: &Vec<(Point, usize)>, map: &mut Map) {
    type Seen = FxHashSet<Point>;
    if map[row][col] == Item::Space {
        return;
    }
    let enemy_type = if map[row][col] == Item::Goblin {
        Item::Elf
    } else {
        Item::Goblin
    };

    let mut seen = Seen::default();
    seen.insert((row, col));
    let mut current: Vec<Vec<Point>> = adj4((row, col)).into_iter().map(|p| vec![p]).collect();
}
