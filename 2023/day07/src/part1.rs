use crate::Hand;
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
    pairs.sort_by_key(|&(hand, _score)| hand);
    dbg!(&pairs);
    pairs
        .iter()
        .enumerate()
        .map(|(i, (_cards, score))| (i + 1) * score)
        .sum();
    0
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
        assert_eq!(process(input), 6440);
    }
}
