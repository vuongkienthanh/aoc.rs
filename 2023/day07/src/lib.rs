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

#[derive(PartialEq, Eq)]
pub enum HandType {
    // same as sorted hand
    HighCard([Card; 5]),
    // [pair, a, b, c],
    OnePair([Card; 4]),
    // [pair1, pair2, a] pair1>pair2
    TwoPair([Card; 3]),
    // [three, a, b]
    ThreeOfAKind([Card; 3]),
    // [three, pair]
    FullHouse([Card; 2]),
    // [four, a]
    FourOfAKind([Card; 2]),
    // single card
    FiveOfAKind(Card),
}
impl HandType {
    pub fn strength(&self) -> usize {
        match self {
            HandType::HighCard(_) => 1,
            HandType::OnePair(_) => 2,
            HandType::TwoPair(_) => 3,
            HandType::ThreeOfAKind(_) => 4,
            HandType::FullHouse(_) => 5,
            HandType::FourOfAKind(_) => 6,
            HandType::FiveOfAKind(_) => 7,
        }
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.strength()
            .partial_cmp(&other.strength())
            .and_then(|ordering| match ordering {
                Ordering::Equal => todo!(),
                order => Some(order),
            })
    }
}
pub struct SortedHand([Card; 5]);

impl SortedHand {
    pub fn from(src: &str) -> Self {
        assert!(src.len() == 5, "hand length should be 5");
        let mut cards = src.chars().map(|c| Card::from(c));
        let mut cards = [
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        ];
        cards.sort_unstable_by(|a, b| b.cmp(a));
        Self(cards)
    }

    pub fn hand_type(&self) -> HandType {
        // five of a kind
        if self.0.iter().take(4).all(|c| c == &self.0[4]) {
            HandType::FiveOfAKind(self.0[0])
        // four of a kind
        } else if self.0.iter().take(3).all(|c| c == &self.0[3]) {
            HandType::FourOfAKind([self.0[0], self.0[4]])
        } else if self.0.iter().skip(1).take(3).all(|c| c == &self.0[4]) {
            HandType::FourOfAKind([self.0[4], self.0[0]])
        // full house
        } else if self.0[0] == self.0[1] && self.0[3] == self.0[4] {
            match self.0[2] == self.0[0] {
                true => HandType::FullHouse([self.0[2], self.0[4]]),
                false => HandType::FullHouse([self.0[2], self.0[0]]),
            }
        // three of a kind
        } else if self.0.iter().take(2).all(|c| c == &self.0[2]) {
            HandType::ThreeOfAKind([self.0[0], self.0[3], self.0[4]])
        } else if self.0.iter().skip(1).take(2).all(|c| c == &self.0[3]) {
            HandType::ThreeOfAKind([self.0[1], self.0[0], self.0[4]])
        } else if self.0.iter().skip(2).take(2).all(|c| c == &self.0[4]) {
            HandType::ThreeOfAKind([self.0[2], self.0[0], self.0[1]])
        // two pair
        } else if self.0[0] == self.0[1] && self.0[2] == self.0[3] {
            HandType::TwoPair([self.0[0], self.0[2], self.0[4]])
        } else if self.0[0] == self.0[1] && self.0[3] == self.0[4] {
            HandType::TwoPair([self.0[0], self.0[3], self.0[2]])
        } else if self.0[1] == self.0[2] && self.0[3] == self.0[4] {
            HandType::TwoPair([self.0[1], self.0[3], self.0[0]])
        // one pair
        } else if self.0[0] == self.0[1] {
            HandType::OnePair([self.0[0], self.0[2], self.0[3], self.0[4]])
        } else if self.0[1] == self.0[2] {
            HandType::OnePair([self.0[1], self.0[0], self.0[3], self.0[4]])
        } else if self.0[2] == self.0[3] {
            HandType::OnePair([self.0[2], self.0[0], self.0[1], self.0[4]])
        } else if self.0[3] == self.0[4] {
            HandType::OnePair([self.0[3], self.0[0], self.0[1], self.0[2]])
        // high card
        } else {
            HandType::HighCard(self.0)
        }
    }
}
