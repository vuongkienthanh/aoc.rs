const LEN: usize = 1_000_000;

pub fn process(_input: &str) -> usize {
    let input: Vec<usize> = _input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize - 1)
        .collect();
    let mut cups_next = [0; LEN];
    for i in 0..LEN - 1 {
        cups_next[i] = i + 1;
    }
    for v in input.windows(2) {
        let (a, b) = (v[0], v[1]);
        cups_next[a] = b;
    }
    cups_next[input.last().cloned().unwrap()] = input.len();
    *cups_next.last_mut().unwrap() = input[0];

    let mut current_cup = input[0];
    for _ in 0..10_000_000 {
        let mut pick_up = vec![];
        let mut cur = current_cup;
        for _ in 0..3 {
            pick_up.push(cups_next[cur]);
            cur = cups_next[cur];
        }
        let mut dst = (current_cup + LEN - 1) % LEN;
        while pick_up.contains(&dst) {
            dst = (dst + LEN - 1) % LEN;
        }
        cups_next[current_cup] = cups_next[pick_up.last().cloned().unwrap()];
        cups_next[pick_up.last().cloned().unwrap()] = cups_next[dst];
        cups_next[dst] = pick_up[0];

        current_cup = cups_next[current_cup];
    }

    let a = cups_next[0];
    let b = cups_next[a];
    (a + 1) * (b + 1)
}
