#[derive(Debug, Clone, Copy)]
pub struct Adj4<T> {
    pub top: Option<T>,
    pub bottom: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,
}
impl<T> IntoIterator for Adj4<T> {
    type Item = Option<T>;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;
    fn into_iter(self) -> Self::IntoIter {
        [self.top, self.bottom, self.left, self.right].into_iter()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Adj8<T> {
    pub top: Option<T>,
    pub bottom: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,
    pub top_left: Option<T>,
    pub top_right: Option<T>,
    pub bottom_left: Option<T>,
    pub bottom_right: Option<T>,
}
impl<T> IntoIterator for Adj8<T> {
    type Item = Option<T>;
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
