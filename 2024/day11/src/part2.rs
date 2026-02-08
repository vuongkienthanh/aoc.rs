use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
struct Key {
    x: usize,
    times: usize,
}
type Memo = HashMap<Key, usize>;

pub fn process(_input: &str, times: usize) -> usize {
    let mut memo: Memo = Memo::new();
    let ans = _input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| blink(x, times, &mut memo))
        .sum();
    ans
}

fn blink(x: usize, times: usize, memo: &mut Memo) -> usize {
    let x_len = x.checked_ilog10().unwrap_or_default() + 1;
    let is_even = x_len % 2 == 0;
    if times == 1 {
        if x == 0 {
            1
        } else if is_even {
            2
        } else {
            1
        }
    } else if let Some(n) = memo.get(&Key { x, times }) {
        *n
    } else if x == 0 {
        let n = blink(1, times - 1, memo);
        memo.insert(Key { x, times }, n);
        n
    } else if is_even {
        let mut left = x;
        let mut right = 0;
        for i in 0..x_len / 2 {
            let digit = left % 10;
            right += digit * 10usize.pow(i);
            left /= 10;
        }
        let n = blink(left, times - 1, memo) + blink(right, times - 1, memo);
        memo.insert(Key { x, times }, n);
        n
    } else {
        let n = blink(x * 2024, times - 1, memo);
        memo.insert(Key { x, times }, n);
        n
    }
}
