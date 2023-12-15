use std::cmp::Ordering;

const CHARSMAP: &str = "AKQJT98765432";

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct Card(usize);

#[derive(PartialEq, Eq, Ord, Clone, Copy)]
pub struct Hand([Card; 5]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

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
impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut sorted = value.0.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));

        if sorted.iter().take(4).all(|c| c == &sorted[4]) {
            HandType::FiveOfAKind
        } else if sorted.iter().take(3).all(|c| c == &sorted[3])
            || sorted.iter().skip(1).take(3).all(|c| c == &sorted[4])
        {
            HandType::FourOfAKind
        } else if sorted[0] == sorted[1]
            && sorted[3] == sorted[4]
            && (sorted[1] == sorted[2] || sorted[2] == sorted[3])
        {
            HandType::FullHouse
        } else if sorted.iter().take(2).all(|c| c == &sorted[2])
            || sorted.iter().skip(1).take(2).all(|c| c == &sorted[3])
            || sorted.iter().skip(2).take(2).all(|c| c == &sorted[4])
        {
            HandType::ThreeOfAKind
        } else if (sorted[0] == sorted[1] && sorted[2] == sorted[3])
            || (sorted[0] == sorted[1] && sorted[3] == sorted[4])
            || (sorted[1] == sorted[2] && sorted[3] == sorted[4])
        {
            HandType::TwoPair
        } else if sorted[0] == sorted[1]
            || sorted[1] == sorted[2]
            || sorted[2] == sorted[3]
            || sorted[3] == sorted[4]
        {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        HandType::from(self)
            .partial_cmp(&HandType::from(other))
            .and_then(|ordering| match ordering {
                Ordering::Equal => self.0.partial_cmp(&other.0),
                other => Some(other),
            })
    }
}
impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx = 12 - self.0;
        write!(f, "{}", CHARSMAP.get(idx..idx + 1).unwrap())
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hand ")?;
        for c in self.0.iter() {
            std::fmt::Debug::fmt(c, f)?;
        }
        Ok(())
    }
}

pub fn process(_input: &str) -> usize {
    let mut pairs = _input
        .lines()
        .map(|line| {
            let mut linesplit = line.split_ascii_whitespace();
            let hand = Hand::from(linesplit.next().unwrap());
            let score = linesplit.next().unwrap().parse::<usize>().unwrap();
            (hand, score)
        })
        .collect::<Vec<_>>();
    pairs.sort_unstable_by_key(|&(hand, _score)| hand);
    pairs
        .iter()
        .enumerate()
        .map(|(i, (_cards, score))| (i + 1) * score)
        .sum::<usize>()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::cmp::Ordering;

    #[test]
    fn test_process() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(process(input), 6440);
    }

    #[rstest]
    #[case("33333", HandType::FiveOfAKind)]
    #[case("9999Q", HandType::FourOfAKind)]
    #[case("Q9999", HandType::FourOfAKind)]
    #[case("66633", HandType::FullHouse)]
    #[case("66333", HandType::FullHouse)]
    #[case("444JT", HandType::ThreeOfAKind)]
    #[case("A444T", HandType::ThreeOfAKind)]
    #[case("AK444", HandType::ThreeOfAKind)]
    #[case("99622", HandType::TwoPair)]
    #[case("99226", HandType::TwoPair)]
    #[case("69922", HandType::TwoPair)]
    #[case("JJT24", HandType::OnePair)]
    #[case("JTT24", HandType::OnePair)]
    #[case("JT224", HandType::OnePair)]
    #[case("JT244", HandType::OnePair)]
    #[case("8A7K2", HandType::HighCard)]
    fn test_handtype(#[case] s: &str, #[case] expected: HandType) {
        assert_eq!(HandType::from(&Hand::from(s)), expected)
    }

    #[rstest]
    #[case("33333", "999Q9", Ordering::Greater)]
    #[case("999Q9", "66336", Ordering::Greater)]
    #[case("66336", "44J4T", Ordering::Greater)]
    #[case("44J4T", "29926", Ordering::Greater)]
    #[case("29926", "JJT24", Ordering::Greater)]
    #[case("JJT24", "8A7K2", Ordering::Greater)]
    fn test_handtype_cmp(#[case] s1: &str, #[case] s2: &str, #[case] expected: Ordering) {
        let h1 = HandType::from(&Hand::from(s1));
        let h2 = HandType::from(&Hand::from(s2));
        assert_eq!(h1.cmp(&h2), expected)
    }
}
