use crate::parsing::parse_input;
use crate::WIN;

pub fn process(_input: &str) -> usize {
    let (numbers, boards) = parse_input(_input);
    let mut boards: Vec<([u8; 25], [bool; 25])> =
        boards.into_iter().map(|v| (v, [false; 25])).collect();
    let mut ans = 0;
    'a: for n in numbers {
        for (board, check) in boards.iter_mut() {
            if let Some(found) = board
                .iter()
                .enumerate()
                .find_map(|(i, b)| (*b == n).then_some(i))
            {
                check[found] = true;
            }
            if WIN.iter().any(|group| group.iter().all(|i| check[*i])) {
                ans = n as usize
                    * board
                        .iter()
                        .zip(check)
                        .filter_map(|(a, b)| (!(*b)).then_some(*a as usize))
                        .sum::<usize>();
                break 'a;
            }
        }
    }
    ans
}