pub fn process(_input: &str) -> usize {
    let input: Vec<usize> = _input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize - 1)
        .collect();
    let len = input.len();
    let mut cups_next = vec![0; input.len()];
    for v in input.windows(2) {
        let (a, b) = (v[0], v[1]);
        cups_next[a] = b;
    }
    cups_next[input.last().cloned().unwrap()] = input.first().cloned().unwrap();
    let mut current_cup = input.first().cloned().unwrap();

    for _ in 0..100 {
        let mut pick_up = vec![];
        let mut cur = current_cup;
        for _ in 0..3 {
            pick_up.push(cups_next[cur]);
            cur = cups_next[cur];
        }
        let mut dst = (current_cup + len - 1) % len;
        while pick_up.contains(&dst) {
            dst = (dst + len - 1) % len;
        }
        cups_next[current_cup] = cups_next[pick_up.last().cloned().unwrap()];
        cups_next[pick_up.last().cloned().unwrap()] = cups_next[dst];
        cups_next[dst] = pick_up.first().cloned().unwrap();

        current_cup = cups_next[current_cup];
    }

    let mut ans = 0;
    let mut cur = 0;
    for _ in 1..len {
        ans = ans * 10 + cups_next[cur] + 1;
        cur = cups_next[cur];
    }
    ans
}
