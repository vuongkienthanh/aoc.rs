use std::cmp::Ordering;

const CHARSMAP: &str = "AKQT98765432J";

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
impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut sorted = value.0.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));

        let handtype = if sorted.iter().take(4).all(|c| c == &sorted[4]) {
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
        };

        // upgrade hands with 'J'
        if value.0.contains(&Card::from('J')) {
            match handtype {
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::TwoPair => {
                    match value.0.iter().filter(|c| c == &&Card::from('J')).count() {
                        1 => HandType::FullHouse,
                        2 => HandType::FourOfAKind,
                        _ => unreachable!(),
                    }
                }
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::FullHouse => HandType::FiveOfAKind,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                other => other,
            }
        } else {
            handtype
        }
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
    // dbg!(&pairs);
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

    #[test]
    fn test_process() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(process(input), 5905);
    }

    #[rstest]
    #[case("33333", HandType::FiveOfAKind)]
    #[case("9999J", HandType::FiveOfAKind)]
    #[case("QJJJJ", HandType::FiveOfAKind)]
    #[case("666JJ", HandType::FiveOfAKind)]
    #[case("66JJJ", HandType::FiveOfAKind)]
    #[case("444JT", HandType::FourOfAKind)]
    #[case("JJJ4T", HandType::FourOfAKind)]
    #[case("99J22", HandType::FullHouse)]
    #[case("99JJ6", HandType::FourOfAKind)]
    #[case("JJT24", HandType::ThreeOfAKind)]
    #[case("JTT24", HandType::ThreeOfAKind)]
    #[case("8A7KJ", HandType::OnePair)]
    fn test_handtype(#[case] s: &str, #[case] expected: HandType) {
        assert_eq!(HandType::from(&Hand::from(s)), expected)
    }
}
