pub mod part1;
pub mod part2;
use std::iter::repeat_n;

type Point = (isize, isize);

fn generate(layer: usize) -> Vec<Point> {
    let l = layer as isize;
    let side = 2 * layer as usize;
    (-l + 1..=l)
        .chain(repeat_n(l, side))
        .chain((-l..=l - 1).rev())
        .chain(repeat_n(-l, side))
        .zip(
            repeat_n(l, side)
                .chain((-l..=l - 1).rev())
                .chain(repeat_n(-l, side))
                .chain(-l + 1..=l),
        )
        .collect()
}
