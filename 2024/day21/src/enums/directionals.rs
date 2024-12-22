use std::collections::HashMap;
type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Directional {
    Blank,
    Up,
    A,
    Left,
    Down,
    Right,
}
impl From<char> for Directional {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            'A' => Self::A,
            _ => panic!(),
        }
    }
}
impl From<Coord> for Directional {
    fn from(value: Coord) -> Self {
        match value {
            (0, 0) => Self::Blank,
            (0, 1) => Self::Up,
            (0, 2) => Self::A,
            (1, 0) => Self::Left,
            (1, 1) => Self::Down,
            (1, 2) => Self::Right,
            _ => panic!(),
        }
    }
}

impl From<Directional> for Coord {
    fn from(value: Directional) -> Self {
        match value {
            Directional::Blank => (0, 0),
            Directional::Up => (0, 1),
            Directional::A => (0, 2),
            Directional::Left => (1, 0),
            Directional::Down => (1, 1),
            Directional::Right => (1, 2),
        }
    }
}
impl Directional {
    pub fn as_arr() -> [Self; 6] {
        [
            Self::Blank,
            Self::A,
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
        ]
    }
}
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DirectionPair(pub Directional, pub Directional);

pub fn directional_paths() -> HashMap<DirectionPair, Vec<Vec<Directional>>> {
    let mut ans = HashMap::<DirectionPair, Vec<Vec<Directional>>>::new();

    for item in Directional::as_arr() {
        for item2 in Directional::as_arr() {
            let (x1, y1) = Coord::from(item);
            let (x2, y2) = Coord::from(item2);
            match (x1 == x2, y1 == y2) {
                (true, true) => {
                    ans.entry(DirectionPair(item, item2))
                        .or_default()
                        .push(vec![]);
                }
                (true, false) => {
                    if y1 > y2 {
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(vec![Directional::Left; y1 - y2]);
                    } else {
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(vec![Directional::Right; y2 - y1]);
                    }
                }

                (false, true) => {
                    if x1 > x2 {
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(vec![Directional::Up; x1 - x2]);
                    } else {
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(vec![Directional::Down; x2 - x1]);
                    }
                }
                (false, false) => match (x1 > x2, y1 > y2) {
                    (true, true) => {
                        let up = vec![Directional::Up; x1 - x2];
                        let left = vec![Directional::Left; y1 - y2];
                        let upleft = Vec::from_iter(up.clone().into_iter().chain(left.clone()));
                        let leftup = Vec::from_iter(left.into_iter().chain(up));
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(upleft);
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(leftup);
                    }
                    (true, false) => {
                        let up = vec![Directional::Up; x1 - x2];
                        let right = vec![Directional::Right; y2 - y1];
                        let upright = Vec::from_iter(up.clone().into_iter().chain(right.clone()));
                        let rightup = Vec::from_iter(right.into_iter().chain(up));
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(upright);
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(rightup);
                    }
                    (false, true) => {
                        let down = vec![Directional::Down; x2 - x1];
                        let left = vec![Directional::Left; y1 - y2];
                        let downleft = Vec::from_iter(down.clone().into_iter().chain(left.clone()));
                        let leftdown = Vec::from_iter(left.into_iter().chain(down));
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(downleft);
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(leftdown);
                    }
                    (false, false) => {
                        let down = vec![Directional::Down; x2 - x1];
                        let right = vec![Directional::Right; y2 - y1];
                        let downright =
                            Vec::from_iter(down.clone().into_iter().chain(right.clone()));
                        let rightdown = Vec::from_iter(right.into_iter().chain(down));
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(downright);
                        ans.entry(DirectionPair(item, item2))
                            .or_default()
                            .push(rightdown);
                    }
                },
            }
        }
    }

    ans
}
