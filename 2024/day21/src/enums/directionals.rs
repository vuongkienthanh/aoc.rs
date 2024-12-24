use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir {
    Blank,
    Up,
    A,
    Left,
    Down,
    Right,
}
impl From<char> for Dir {
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
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DPair(pub Dir, pub Dir);

pub fn directional_paths() -> HashMap<DPair, Vec<Vec<Dir>>> {
    let mut ans = HashMap::<DPair, Vec<Vec<Dir>>>::new();

    for item in [
        Dir::Blank,
        Dir::A,
        Dir::Up,
        Dir::Down,
        Dir::Left,
        Dir::Right,
    ] {
        ans.insert(DPair(item, item), vec![vec![]]);
    }
    for (a, b, sequences) in [
        (Dir::Up, Dir::A, vec![">"]),
        (Dir::Up, Dir::Left, vec!["v<"]),
        (Dir::Up, Dir::Down, vec!["v"]),
        (Dir::Up, Dir::Right, vec!["v>", ">v"]),
        (Dir::A, Dir::Up, vec!["<"]),
        (Dir::A, Dir::Left, vec!["v<<"]),
        (Dir::A, Dir::Down, vec!["v<", "<v"]),
        (Dir::A, Dir::Right, vec!["v"]),
        (Dir::Left, Dir::Up, vec![">^"]),
        (Dir::Left, Dir::A, vec![">>^"]),
        (Dir::Left, Dir::Down, vec![">"]),
        (Dir::Left, Dir::Right, vec![">>"]),
        (Dir::Down, Dir::Up, vec!["^"]),
        (Dir::Down, Dir::A, vec!["^>", ">^"]),
        (Dir::Down, Dir::Left, vec!["<"]),
        (Dir::Down, Dir::Right, vec![">"]),
        (Dir::Right, Dir::Up, vec!["<^", "^<"]),
        (Dir::Right, Dir::A, vec!["^"]),
        (Dir::Right, Dir::Left, vec!["<<"]),
        (Dir::Right, Dir::Down, vec!["<"]),
    ] {
        ans.insert(
            DPair(a, b),
            sequences
                .into_iter()
                .map(|sequence| sequence.chars().map(Dir::from).collect::<Vec<_>>())
                .collect(),
        );
    }

    ans
}
