pub fn mask(x: u8) -> u64 {
    match x {
        b'a'..=b'z' => 1 << ((x - b'a') as u64 + 1),
        b'A'..=b'Z' => 1 << ((x - b'A') as u64 + 27),
        _ => panic!(),
    }
}

pub fn process(_input: &str) -> u32 {
    _input
        .lines()
        .map(|line| {
            let mut seen = 0u64;
            let mid = line.len() / 2;
            let mut ans = 0;
            for x in line[0..mid].bytes() {
                seen |= mask(x);
            }
            for x in line[mid..].bytes() {
                let bitmask = mask(x);
                if seen & bitmask == bitmask {
                    ans = bitmask.ilog(2);
                    break;
                }
            }
            ans
        })
        .sum()
}
