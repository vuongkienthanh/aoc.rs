use std::collections::{HashMap, VecDeque};
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
    #[rustfmt::skip]
    pub fn adj(&self) -> Vec<(Action, Self)> {
        let mut ans = vec![];
        let c: Coord = (*self).into();
        if c.0 > 0 { ans.push((Action::MoveUp, (c.0 - 1, c.1).into())); }
        if c.1 > 0 { ans.push((Action::MoveLeft, (c.0, c.1 - 1).into())); }
        if c.0 < 3 { ans.push((Action::MoveDown, (c.0 + 1, c.1).into())); }
        if c.1 < 2 { ans.push((Action::MoveRight, (c.0, c.1 + 1).into())); }
        ans
    }

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Directional {
    Blank,
    Up,
    A,
    Left,
    Down,
    Right,
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

#[derive(Debug, Clone, Copy)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PressA,
}

fn manhattan(a: Coord, b: Coord) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct NumericPair(Numeric, Numeric);

pub fn numeric_paths() -> HashMap<NumericPair, Vec<VecDeque<Action>>> {
    let mut ans = HashMap::new();
    for item in Numeric::as_arr() {
        for (action, adj) in item.adj() {
            if item != Numeric::Blank && adj != Numeric::Blank {
                ans.insert(NumericPair(item, adj), vec![VecDeque::from([action])]);
            }
        }
    }

    for item in Numeric::as_arr() {
        for item2 in Numeric::as_arr() {
            if item != item2 {
                build_path(item, item2, &mut ans);
            }
        }
    }

    ans
}

fn build_path(
    item: Numeric,
    item2: Numeric,
    memo: &mut HashMap<NumericPair, Vec<VecDeque<Action>>>,
) -> Vec<VecDeque<Action>> {
    if let Some(ans) = memo.get(&NumericPair(item, item2)) {
        ans.clone()
    } else {
        let len = manhattan(item.into(), item2.into());
        let mut ans = vec![];
        for (action, adj) in item
            .adj()
            .into_iter()
            .filter(|(_, adj)| manhattan((*adj).into(), item2.into()) < len)
        {
            for mut vec_action in build_path(adj, item2, memo) {
                vec_action.push_front(action);
                ans.push(vec_action);
            }
        }
        if item != Numeric::Blank && item2 != Numeric::Blank {
            memo.insert(NumericPair(item, item2), ans.clone());
        }
        ans
    }
}
