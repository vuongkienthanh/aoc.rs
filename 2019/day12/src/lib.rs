pub mod parsing;
pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Axis {
    pos: isize,
    vel: isize,
}

struct Moon(Axis, Axis, Axis);

impl Moon {
    fn new(pos: (isize, isize, isize)) -> Self {
        Moon(
            Axis { pos: pos.0, vel: 0 },
            Axis { pos: pos.1, vel: 0 },
            Axis { pos: pos.2, vel: 0 },
        )
    }
}
