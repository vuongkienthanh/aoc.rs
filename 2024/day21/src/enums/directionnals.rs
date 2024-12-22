use crate::enums::Action;
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
impl From<Action> for Directional {
    fn from(value: Action) -> Self {
        match value {
            Action::MoveUp => Self::Up,
            Action::MoveDown => Self::Down,
            Action::MoveLeft => Self::Left,
            Action::MoveRight => Self::Right,
            Action::PressA => Self::A,
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
