pub mod part1;
pub mod part2;

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct Adj4 {
    top: Option<Coord>,
    bottom: Option<Coord>,
    left: Option<Coord>,
    right: Option<Coord>,
}
impl IntoIterator for Adj4 {
    type Item = Option<Coord>;

    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [self.top, self.bottom, self.left, self.right].into_iter()
    }
}

pub fn adj4(c: Coord) -> Adj4 {
    Adj4 {
        top: if c.0 > 0 { Some((c.0 - 1, c.1)) } else { None },
        bottom: Some((c.0 + 1, c.1)),
        left: if c.1 > 0 { Some((c.0, c.1 - 1)) } else { None },
        right: Some((c.0, c.1 + 1)),
    }
}

fn is_wall(input: usize, coord: Coord) -> bool {
    let n = (coord.0 + coord.1).pow(2u32) + 3 * coord.0 + coord.1 + input;
    n.count_ones() % 2 != 0
}
