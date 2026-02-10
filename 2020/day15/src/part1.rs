pub fn process(_input: &str) -> usize {
    find(_input, 2020)
}

pub fn find(input: &str, at: usize) -> usize {
    let mut seen: Vec<_> = vec![None; at];
    let input: Vec<_> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    for (turn, c) in input.iter().enumerate().take(input.len() - 1) {
        *seen.get_mut(*c).unwrap() = Some(turn + 1);
    }
    let mut last_spoken = input.last().cloned().unwrap();

    for i in input.len() + 1..=at {
        match seen[last_spoken] {
            None => {
                seen[last_spoken] = Some(i - 1);
                last_spoken = 0;
            }
            Some(turn) => {
                seen[last_spoken] = Some(i - 1);
                last_spoken = i - 1 - turn;
            }
        }
    }

    last_spoken
}
