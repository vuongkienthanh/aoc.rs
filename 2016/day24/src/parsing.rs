use crate::adj::adj4;
use grid::Grid;
use std::collections::HashMap;

pub type Point = (usize, usize);
pub type G = Grid<Item>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Item {
    Wall,
    Number(char),
    Space,
    Cross,
}

pub fn parse_input(input: &str) -> (Grid<Item>, HashMap<char, Point>) {
    let mut number_locations = HashMap::new();
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
    let mut g: Grid<Item> = ans.into();
    for i in 0..g.rows() {
        for j in 0..g.cols() {
            if g[(i, j)] != Item::Space {
                continue;
            }
            match adj4((i, j))
                .into_iter()
                .filter(|x| g[*x] != Item::Wall)
                .count()
            {
                3 | 4 => *g.get_mut(i, j).unwrap() = Item::Cross,
                _ => (),
            }
        }
    }
    (g, number_locations)
}
