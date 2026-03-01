use crate::parsing::parse_input;
use crate::part1::Hand;

impl Hand {
    fn need_win(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
    fn need_draw(&self) -> Hand {
        self.clone()
    }
    fn need_lose(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}
pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut score = 0;
    for (l, r) in input {
        let l: Hand = l.into();
        match r {
            'X' => score += l.need_lose().score(),
            'Y' => score += l.need_draw().score() + 3,
            'Z' => score += l.need_win().score() + 6,
            _ => panic!(),
        }
    }
    score
}
