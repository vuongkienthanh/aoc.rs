pub mod part1;
pub mod part2;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Tile {
    empty,
    wall,
    block,
    paddle,
    ball,
}
impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Tile::empty,
            1 => Tile::wall,
            2 => Tile::block,
            3 => Tile::paddle,
            4 => Tile::ball,
            _ => panic!("should be 0 1 2 3 4"),
        }
    }
}
impl Tile {
    pub fn is_indestructible(&self) -> bool {
        matches!(self, Tile::wall | Tile::paddle)
    }
}
