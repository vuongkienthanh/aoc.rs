use crate::parsing::parse_input;

#[derive(Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}
impl From<char> for Hand {
    fn from(value: char) -> Hand {
        match value {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!(),
        }
    }
}
impl Hand {
    pub fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

pub fn process(_input: &str) -> usize {
    use Hand::*;
    let input = parse_input(_input);
    let mut score = 0;
    for (l, r) in input {
        let l: Hand = l.into();
        let r: Hand = r.into();
        match (l, &r) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => score += r.score(),
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => score += r.score() + 3,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => score += r.score() + 6,
        }
    }
    score
}
