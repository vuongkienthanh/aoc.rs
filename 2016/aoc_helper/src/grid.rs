pub type Coord = (usize, usize);

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

pub fn adj4(i: usize, j: usize, rows: usize, cols: usize) -> Adj4 {
    Adj4 {
        top: if i > 0 { Some((i - 1, j)) } else { None },
        bottom: if i < rows - 1 { Some((i + 1, j)) } else { None },
        left: if j > 0 { Some((i, j - 1)) } else { None },
        right: if j < cols - 1 { Some((i, j + 1)) } else { None },
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Adj8 {
    top: Option<Coord>,
    bottom: Option<Coord>,
    left: Option<Coord>,
    right: Option<Coord>,
    top_left: Option<Coord>,
    top_right: Option<Coord>,
    bottom_left: Option<Coord>,
    bottom_right: Option<Coord>,
}
impl IntoIterator for Adj8 {
    type Item = Option<Coord>;

    type IntoIter = std::array::IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        [
            self.top,
            self.bottom,
            self.left,
            self.right,
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
        .into_iter()
    }
}

pub fn adj8(i: usize, j: usize, rows: usize, cols: usize) -> Adj8 {
    let (
        mut top,
        mut bottom,
        mut left,
        mut right,
        mut top_left,
        mut top_right,
        mut bottom_left,
        mut bottom_right,
    ) = (None, None, None, None, None, None, None, None);
    if i > 0 {
        top = Some((i - 1, j));
        if j > 0 {
            top_left = Some((i - 1, j - 1));
        }
        if j < cols - 1 {
            top_right = Some((i - 1, j + 1));
        }
    }
    if i < rows - 1 {
        bottom = Some((i + 1, j));
        if j > 0 {
            bottom_left = Some((i + 1, j - 1));
        }
        if j < cols - 1 {
            bottom_right = Some((i + 1, j + 1));
        }
    }
    if j > 0 {
        left = Some((i, j - 1));
    }
    if j < cols - 1 {
        right = Some((i, j + 1));
    }
    Adj8 {
        top,
        bottom,
        left,
        right,
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    }
}
