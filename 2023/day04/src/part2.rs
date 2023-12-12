use std::collections::{HashMap, HashSet};
pub fn process(_input: &str) -> usize {
    let mut scratch_cards: HashMap<usize, usize> = HashMap::new();
    for line in _input.lines() {
        let mut row = line.split(": ");
        let card_id = row
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        *scratch_cards.entry(card_id).or_default() += 1;

        let mut content = row.next().unwrap().split(" | ");
        let winning_numbers = content
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<HashSet<_>>();

        let amount_of_won_cards = content
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .filter(|n| winning_numbers.contains(n))
            .count();
        let card_id_count = scratch_cards.get(&card_id).cloned().unwrap();

        for i in (card_id + 1)..=(card_id + amount_of_won_cards) {
            *scratch_cards.entry(i).or_default() += card_id_count;
        }
    }
    scratch_cards.values().sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(process(input), 30);
    }
}
