use crate::enums::directionals::Directional;
use std::collections::HashMap;
type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Numeric {
    Blank,
    Zero,
    A,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl From<char> for Numeric {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::A,
            _ => panic!(),
        }
    }
}

impl From<Coord> for Numeric {
    fn from(value: Coord) -> Self {
        match value {
            (0, 0) => Self::Seven,
            (0, 1) => Self::Eight,
            (0, 2) => Self::Nine,
            (1, 0) => Self::Four,
            (1, 1) => Self::Five,
            (1, 2) => Self::Six,
            (2, 0) => Self::One,
            (2, 1) => Self::Two,
            (2, 2) => Self::Three,
            (3, 0) => Self::Blank,
            (3, 1) => Self::Zero,
            (3, 2) => Self::A,
            _ => panic!(),
        }
    }
}
impl From<Numeric> for Coord {
    fn from(value: Numeric) -> Self {
        match value {
            Numeric::Blank => (3, 0),
            Numeric::Zero => (3, 1),
            Numeric::A => (3, 2),
            Numeric::One => (2, 0),
            Numeric::Two => (2, 1),
            Numeric::Three => (2, 2),
            Numeric::Four => (1, 0),
            Numeric::Five => (1, 1),
            Numeric::Six => (1, 2),
            Numeric::Seven => (0, 0),
            Numeric::Eight => (0, 1),
            Numeric::Nine => (0, 2),
        }
    }
}

impl Numeric {
    pub fn as_arr() -> [Self; 12] {
        [
            Self::Blank,
            Self::Zero,
            Self::A,
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
        ]
    }
}
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct NumericPair(pub Numeric, pub Numeric);

pub fn numeric_paths() -> HashMap<NumericPair, Vec<Vec<Directional>>> {
    let mut ans = HashMap::<NumericPair, Vec<Vec<Directional>>>::new();

    for item in Numeric::as_arr() {
        for item2 in Numeric::as_arr() {
            let (x1, y1) = Coord::from(item);
            let (x2, y2) = Coord::from(item2);
            match (x1 == x2, y1 == y2) {
                (true, true) => {
                    ans.entry(NumericPair(item, item2))
                        .or_default()
                        .push(vec![]);
                }
                (true, false) => {
                    if y1 > y2 {
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(vec![Directional::Left; y1 - y2]);
                    } else {
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(vec![Directional::Right; y2 - y1]);
                    }
                }

                (false, true) => {
                    if x1 > x2 {
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(vec![Directional::Up; x1 - x2]);
                    } else {
                        ans.entry(NumericPair(item, item2))
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
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(upleft);
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(leftup);
                    }
                    (true, false) => {
                        let up = vec![Directional::Up; x1 - x2];
                        let right = vec![Directional::Right; y2 - y1];
                        let upright = Vec::from_iter(up.clone().into_iter().chain(right.clone()));
                        let rightup = Vec::from_iter(right.into_iter().chain(up));
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(upright);
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(rightup);
                    }
                    (false, true) => {
                        let down = vec![Directional::Down; x2 - x1];
                        let left = vec![Directional::Left; y1 - y2];
                        let downleft = Vec::from_iter(down.clone().into_iter().chain(left.clone()));
                        let leftdown = Vec::from_iter(left.into_iter().chain(down));
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(downleft);
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(leftdown);
                    }
                    (false, false) => {
                        let down = vec![Directional::Down; x2 - x1];
                        let right = vec![Directional::Right; y2 - y1];
                        let downright =
                            Vec::from_iter(down.clone().into_iter().chain(right.clone()));
                        let rightdown = Vec::from_iter(right.into_iter().chain(down));
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(downright);
                        ans.entry(NumericPair(item, item2))
                            .or_default()
                            .push(rightdown);
                    }
                },
            }
        }
    }

    ans
}
