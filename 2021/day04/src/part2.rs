use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (numbers, boards) = parse_input(_input);
    let mut boards: Vec<([u8; 25], [bool; 25])> =
        boards.into_iter().map(|v| (v, [false; 25])).collect();
    let mut ans = 0;
    for n in numbers {
        for (board, check) in boards.extract_if(.., |(board, check)| {
            if let Some(found) = board
                .iter()
                .enumerate()
                .find_map(|(i, b)| (*b == n).then_some(i))
            {
                check[found] = true;
            }
            WIN.iter().any(|group| group.iter().all(|i| check[*i]))
        }) {
            ans = n as usize
                * board
                    .into_iter()
                    .zip(check)
                    .filter_map(|(a, b)| (!b).then_some(a as usize))
                    .sum::<usize>();
        }
    }
    ans
}
