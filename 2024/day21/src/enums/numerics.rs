use crate::enums::directionals::Dir;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Num {
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
impl From<char> for Num {
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

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct NPair(pub Num, pub Num);

pub fn numeric_paths() -> HashMap<NPair, Vec<Vec<Dir>>> {
    let mut ans = HashMap::<NPair, Vec<Vec<Dir>>>::new();

    for item in [
        Num::Blank,
        Num::Zero,
        Num::A,
        Num::One,
        Num::Two,
        Num::Three,
        Num::Four,
        Num::Five,
        Num::Six,
        Num::Seven,
        Num::Eight,
        Num::Nine,
    ] {
        ans.insert(NPair(item, item), vec![vec![]]);
    }
    for (a, b, sequences) in [
        // 7
        (Num::Seven, Num::Eight, vec![">"]),
        (Num::Seven, Num::Nine, vec![">>"]),
        (Num::Seven, Num::Four, vec!["v"]),
        (Num::Seven, Num::Five, vec!["v>", ">v"]),
        (Num::Seven, Num::Six, vec!["v>>", ">>v"]),
        (Num::Seven, Num::One, vec!["vv"]),
        (Num::Seven, Num::Two, vec!["vv>", ">vv"]),
        (Num::Seven, Num::Three, vec!["vv>>", ">>vv"]),
        (Num::Seven, Num::Zero, vec![">vvv"]),
        (Num::Seven, Num::A, vec![">>vvv"]),
        // 8
        (Num::Eight, Num::Seven, vec!["<"]),
        (Num::Eight, Num::Nine, vec![">"]),
        (Num::Eight, Num::Four, vec!["v<", ">v"]),
        (Num::Eight, Num::Five, vec!["v"]),
        (Num::Eight, Num::Six, vec!["v>", ">v"]),
        (Num::Eight, Num::One, vec!["vv<", "<vv"]),
        (Num::Eight, Num::Two, vec!["vv"]),
        (Num::Eight, Num::Three, vec!["vv>", ">vv"]),
        (Num::Eight, Num::Zero, vec!["vvv"]),
        (Num::Eight, Num::A, vec!["vvv>", ">vvv"]),
        // 9
        (Num::Nine, Num::Seven, vec!["<<"]),
        (Num::Nine, Num::Eight, vec!["<"]),
        (Num::Nine, Num::Four, vec!["<<v", "v<<"]),
        (Num::Nine, Num::Five, vec!["<v", "v<"]),
        (Num::Nine, Num::Six, vec!["v"]),
        (Num::Nine, Num::One, vec!["<<vv", "vv<<"]),
        (Num::Nine, Num::Two, vec!["<vv", "vv<"]),
        (Num::Nine, Num::Three, vec!["vv"]),
        (Num::Nine, Num::Zero, vec!["vvv<", "<vvv"]),
        (Num::Nine, Num::A, vec!["vvv"]),
        // 4
        (Num::Four, Num::Seven, vec!["^"]),
        (Num::Four, Num::Eight, vec!["^>", ">^"]),
        (Num::Four, Num::Nine, vec!["^>>", ">>^"]),
        (Num::Four, Num::Five, vec![">"]),
        (Num::Four, Num::Six, vec![">>"]),
        (Num::Four, Num::One, vec!["v"]),
        (Num::Four, Num::Two, vec!["v>", ">v"]),
        (Num::Four, Num::Three, vec!["v>>", ">>v"]),
        (Num::Four, Num::Zero, vec![">vv"]),
        (Num::Four, Num::A, vec![">>vv"]),
        // 5
        (Num::Five, Num::Seven, vec!["^<", "<^"]),
        (Num::Five, Num::Eight, vec!["^"]),
        (Num::Five, Num::Nine, vec!["^>", ">^"]),
        (Num::Five, Num::Four, vec!["<"]),
        (Num::Five, Num::Six, vec![">"]),
        (Num::Five, Num::One, vec!["v<", "<v"]),
        (Num::Five, Num::Two, vec!["v"]),
        (Num::Five, Num::Three, vec!["v>", ">v"]),
        (Num::Five, Num::Zero, vec!["vv"]),
        (Num::Five, Num::A, vec!["vv>", ">vv"]),
        // 6
        (Num::Six, Num::Seven, vec!["^<<", "<<^"]),
        (Num::Six, Num::Eight, vec!["^<", "<^"]),
        (Num::Six, Num::Nine, vec!["^"]),
        (Num::Six, Num::Four, vec!["<<"]),
        (Num::Six, Num::Five, vec!["<"]),
        (Num::Six, Num::One, vec!["v<<", "<<^"]),
        (Num::Six, Num::Two, vec!["v<", "<v"]),
        (Num::Six, Num::Three, vec!["v"]),
        (Num::Six, Num::Zero, vec!["vv<", "<vv"]),
        (Num::Six, Num::A, vec!["vv"]),
        // 1
        (Num::One, Num::Seven, vec!["^^"]),
        (Num::One, Num::Eight, vec!["^^>", ">^^"]),
        (Num::One, Num::Nine, vec!["^^>>", ">>^^"]),
        (Num::One, Num::Four, vec!["^"]),
        (Num::One, Num::Five, vec!["^>", ">^"]),
        (Num::One, Num::Six, vec!["^>>", ">>^"]),
        (Num::One, Num::Two, vec![">"]),
        (Num::One, Num::Three, vec![">>"]),
        (Num::One, Num::Zero, vec![">v"]),
        (Num::One, Num::A, vec![">>v"]),
        // 2
        (Num::Two, Num::Seven, vec!["^^<", "<^^"]),
        (Num::Two, Num::Eight, vec!["^^"]),
        (Num::Two, Num::Nine, vec!["^^>", ">^^"]),
        (Num::Two, Num::Four, vec!["^<", "<^"]),
        (Num::Two, Num::Five, vec!["^"]),
        (Num::Two, Num::Six, vec!["^>", ">^"]),
        (Num::Two, Num::One, vec!["<"]),
        (Num::Two, Num::Three, vec![">"]),
        (Num::Two, Num::Zero, vec!["v"]),
        (Num::Two, Num::A, vec!["v>", ">v"]),
        // 3
        (Num::Three, Num::Seven, vec!["^^<<", "<<^^"]),
        (Num::Three, Num::Eight, vec!["^^<", "<^^"]),
        (Num::Three, Num::Nine, vec!["^^"]),
        (Num::Three, Num::Four, vec!["^<<", "<<^"]),
        (Num::Three, Num::Five, vec!["^<", "<^"]),
        (Num::Three, Num::Six, vec!["^"]),
        (Num::Three, Num::One, vec!["<<"]),
        (Num::Three, Num::Two, vec!["<"]),
        (Num::Three, Num::Zero, vec!["v<", "<v"]),
        (Num::Three, Num::A, vec!["v"]),
        // 0
        (Num::Zero, Num::Seven, vec!["^^^<"]),
        (Num::Zero, Num::Eight, vec!["^^^"]),
        (Num::Zero, Num::Nine, vec!["^^^>", ">^^^"]),
        (Num::Zero, Num::Four, vec!["^^<"]),
        (Num::Zero, Num::Five, vec!["^^"]),
        (Num::Zero, Num::Six, vec!["^^>", ">^^"]),
        (Num::Zero, Num::One, vec!["^<"]),
        (Num::Zero, Num::Two, vec!["^"]),
        (Num::Zero, Num::Three, vec!["^>", ">^"]),
        (Num::Zero, Num::A, vec![">"]),
        // A
        (Num::A, Num::Seven, vec!["^^^<<"]),
        (Num::A, Num::Eight, vec!["^^^<", "<^^^"]),
        (Num::A, Num::Nine, vec!["^^^"]),
        (Num::A, Num::Four, vec!["^^<<"]),
        (Num::A, Num::Five, vec!["^^<", "<^^"]),
        (Num::A, Num::Six, vec!["^^"]),
        (Num::A, Num::One, vec!["^<<"]),
        (Num::A, Num::Two, vec!["^<", "<^"]),
        (Num::A, Num::Three, vec!["^"]),
        (Num::A, Num::Zero, vec!["<"]),
    ] {
        ans.insert(
            NPair(a, b),
            sequences
                .into_iter()
                .map(|sequence| sequence.chars().map(Dir::from).collect::<Vec<_>>())
                .collect(),
        );
    }

    ans
}
