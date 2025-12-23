pub mod part1;
pub mod part2;
use std::iter::repeat_n;

type Point = (isize, isize);

struct Spiral {
    layer: isize,
}

impl Spiral {
    fn next(&mut self) -> Vec<Point> {
        let ans = Spiral::generate(self.layer);
        self.layer += 1;
        ans
    }

    fn generate(l: isize) -> Vec<Point> {
        if l == 0 {
            return vec![(0, 0)];
        }
        let side = 2 * l as usize;
        (-l + 1..=l)
            .chain(repeat_n(l, side))
            .chain((-l..=l - 1).rev())
            .chain(repeat_n(-l, side))
            .zip(
                repeat_n(l, side)
                    .chain((-l..=l - 1).rev())
                    .chain(repeat_n(-l, side))
                    .chain(-l + 1..l),
            )
            .collect()
    }
}
