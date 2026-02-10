use crate::{Point3D, min_max_3d, parse_3d};
pub fn process(_input: &str) -> usize {
    let map = parse_3d(_input);
    let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = min_max_3d(&map);
    println!("{map:?}");
    println!("{min_x} {max_x} {min_y} {max_y} {min_z} {max_z}");
    todo!("part1");
}
