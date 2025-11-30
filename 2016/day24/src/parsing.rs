use grid::Grid;
use std::collections::BTreeMap;

pub type Point = (usize, usize);
pub type G = Grid<Item>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Item {
    Wall,
    Space,
    Number(char),
}

pub fn parse_input(input: &str) -> (Grid<Item>, BTreeMap<char, Point>) {
    let mut number_locations = BTreeMap::new();
    let mut ans: Vec<Vec<Item>> = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut v = vec![];
        for (j, c) in line.char_indices() {
            match c {
                '#' => v.push(Item::Wall),
                '.' => v.push(Item::Space),
                '0'..='9' => {
                    number_locations.insert(c, (i, j));
                    v.push(Item::Number(c));
                }
                _ => (),
            }
        }
        ans.push(v);
    }
    (ans.into(), number_locations)
}
