pub use crate::{Card, Hand, HandType};
use std::cmp::Ordering;

const CHARSMAP: &str = "AKQT98765432J";

impl From<char> for Card {
    fn from(value: char) -> Self {
        CHARSMAP
            .chars()
            .zip((0..13).rev())
            .find_map(|(s, i)| (s == value).then_some(Card(i)))
            .expect("parse card error")
    }
}
impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        assert!(value.len() == 5, "hand length should be 5");
        let mut cards = value.chars().map(|c| Card::from(c));
        let cards = [
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        ];
        Self(cards)
    }
}

pub fn process(_input: &str) -> usize {
    todo!("part2")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(process(input), 5905);
    }
}
