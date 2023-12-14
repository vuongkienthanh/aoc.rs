pub mod part1;
pub mod part2;

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

