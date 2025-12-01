pub fn process(_input: &str) -> usize {
    // the idea is
    // the first part of assembly about the lower bound of some input
    // register a plus something ( optimization same as day 12)
    // then continuosly divide by 2
    // if odd send signal 1
    // if even send signal 0
    let lower_bound = _input
        .lines()
        .skip(1)
        .take(2)
        .map(|line| {
            line.split(' ')
                .nth(1)
                .map(|d: &str| d.parse::<usize>().unwrap())
                .unwrap()
        })
        .product::<usize>();

    let mut ans = 0;
    loop {
        // odd
        ans = ans * 2 + 1;
        // even (even at second as puzzle require out 0 first)
        ans *= 2;
        if ans >= lower_bound {
            return ans - lower_bound;
        }
    }
}
