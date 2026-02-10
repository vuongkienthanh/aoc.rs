pub mod part1;
pub mod part2;
use fxhash::FxHashMap;

pub type Point3D = (isize, isize, isize);
pub type MinMax = (isize, isize);
#[derive(Debug)]
pub enum Cell {
    Active,
    Inactive,
}

fn parse_3d(input: &str) -> FxHashMap<Point3D, Cell> {
    let mut map = FxHashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            map.insert(
                (x as isize, y as isize, 0),
                match cell {
                    '#' => Cell::Active,
                    '.' => Cell::Inactive,
                    _ => panic!(),
                },
            );
        }
    }
    map
}
fn min_max_3d(map: &FxHashMap<Point3D, Cell>) -> (MinMax, MinMax, MinMax) {
    let (mut min_x, mut max_x) = (isize::MAX, isize::MIN);
    let (mut min_y, mut max_y) = (isize::MAX, isize::MIN);
    let (mut min_z, mut max_z) = (isize::MAX, isize::MIN);
    for (x, y, z) in map.keys() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
        min_z = min_z.min(*z);
        max_z = max_z.max(*z);
    }

    (
        (min_x - 1, max_x + 1),
        (min_y - 1, max_y + 1),
        (min_z - 1, max_z + 1),
    )
}
