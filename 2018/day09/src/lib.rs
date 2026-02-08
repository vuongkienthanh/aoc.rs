pub mod part1;
pub mod part2;
pub mod parsing;

fn game(players: usize, max: usize) -> usize {
    #[derive(Clone)]
    struct Marble {
        prev: usize,
        next: usize,
    }
    let mut map = vec![Marble { prev: 0, next: 0 }; max + 1];
    let mut scores = vec![0; players];
    let mut current = 0;

    for (i, player) in (1..=max).zip((0..players).cycle()) {
        if i % 23 == 0 {
            scores[player] += i;
            for _ in 0..7 {
                current = map[current].prev;
            }
            scores[player] += current;
            let left = map[current].prev;
            let right = map[current].next;
            map[left].next = right;
            map[right].prev = left;
            current = right;
        } else {
            let left = map[current].next;
            let right = map[left].next;
            map[left].next = i;
            map[right].prev = i;
            map[i].prev = left;
            map[i].next = right;
            current = i;
        }
    }
    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(9, 25, 32)]
    #[case(10, 1618, 8317)]
    #[case(13, 7999, 146373)]
    #[case(17, 1104, 2764)]
    #[case(21, 6111, 54718)]
    #[case(30, 5807, 37305)]
    fn test_game(#[case] players: usize, #[case] max: usize, #[case] expect: usize) {
        assert_eq!(game(players, max), expect);
    }
}
