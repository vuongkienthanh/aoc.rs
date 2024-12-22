pub mod part1;
pub mod part2;
pub mod part2_corners;

use grid::Grid;
fn parse(input: &str) -> Grid<char> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>()
        .into()
}

type Coord = (usize, usize);

#[derive(Eq, PartialEq, Hash)]
struct CoordKey {
    i: usize,
    j: usize,
}

#[derive(Clone)]
struct Adj {
    up: Option<Coord>,
    down: Option<Coord>,
    left: Option<Coord>,
    right: Option<Coord>,
}
impl IntoIterator for Adj {
    type Item = Option<Coord>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        [self.up, self.down, self.left, self.right]
            .to_vec()
            .into_iter()
    }
}

fn adj4(i: usize, j: usize, rows: usize, cols: usize) -> Adj {
    Adj {
        up: if i > 0 { Some((i - 1, j)) } else { None },
        down: if i < rows - 1 { Some((i + 1, j)) } else { None },
        left: if j > 0 { Some((i, j - 1)) } else { None },
        right: if j < cols - 1 { Some((i, j + 1)) } else { None },
    }
}
