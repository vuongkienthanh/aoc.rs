use std::cmp::Ordering;
pub mod part1;
pub mod part2;

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct Card(usize);

impl Card {
    pub fn from(c: char) -> Self {
        "AKQJT98765432"
            .chars()
            .zip((0..13).rev())
            .find_map(|(s, i)| (s == c).then_some(Card(i)))
            .expect("parse card error")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(PartialEq, Eq, Ord, Clone, Copy)]
pub struct Hand([Card; 5]);
#[derive(PartialEq, Eq, Ord, Clone, Copy)]
pub struct SortedHand([Card; 5]);

impl Hand {
    pub fn from(src: &str) -> Self {
        assert!(src.len() == 5, "hand length should be 5");
        let mut cards = src.chars().map(|c| Card::from(c));
        let cards = [
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        ];
        Self(cards)
    }

    pub fn to_sorted(&self) -> SortedHand {
        let mut sorted_cards = self.0.clone();
        sorted_cards.sort_unstable_by(|a, b| b.cmp(a));
        SortedHand(sorted_cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand_type()
            .partial_cmp(&other.hand_type())
            .and_then(|ordering| match ordering {
                Ordering::Equal => self.0.partial_cmp(&other.0),
                other => Some(other),
            })
    }
}
impl PartialOrd for SortedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand_type()
            .partial_cmp(&other.hand_type())
            .and_then(|ordering| match ordering {
                Ordering::Equal => self.0.partial_cmp(&other.0),
                other => Some(other),
            })
    }
}

pub trait ToHandType {
    fn hand_type(&self) -> HandType;
}
impl ToHandType for Hand {
    fn hand_type(&self) -> HandType {
        self.to_sorted().hand_type()
    }
}

impl ToHandType for SortedHand {
    fn hand_type(&self) -> HandType {
        if self.0.iter().take(4).all(|c| c == &self.0[4]) {
            HandType::FiveOfAKind
        } else if self.0.iter().take(3).all(|c| c == &self.0[3])
            || self.0.iter().skip(1).take(3).all(|c| c == &self.0[4])
        {
            HandType::FourOfAKind
        } else if self.0[0] == self.0[1] && self.0[3] == self.0[4] {
            HandType::FullHouse
        } else if self.0.iter().take(2).all(|c| c == &self.0[2])
            || self.0.iter().skip(1).take(2).all(|c| c == &self.0[3])
            || self.0.iter().skip(2).take(2).all(|c| c == &self.0[4])
        {
            HandType::ThreeOfAKind
        } else if (self.0[0] == self.0[1] && self.0[2] == self.0[3])
            || (self.0[0] == self.0[1] && self.0[3] == self.0[4])
            || (self.0[1] == self.0[2] && self.0[3] == self.0[4])
        {
            HandType::TwoPair
        } else if self.0[0] == self.0[1]
            || self.0[1] == self.0[2]
            || self.0[2] == self.0[3]
            || self.0[3] == self.0[4]
        {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}
