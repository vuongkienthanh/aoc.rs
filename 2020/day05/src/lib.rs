pub mod part1;
pub mod part2;

fn parse(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| {
            let mut i = line.bytes();
            let mut ans = 0;
            for c in i.by_ref().take(7) {
                match c {
                    b'F' => ans <<= 1,
                    b'B' => ans = (ans << 1) | 1,
                    _ => panic!(),
                }
            }
            for c in i {
                match c {
                    b'L' => ans <<= 1,
                    b'R' => ans = (ans << 1) | 1,
                    _ => panic!(),
                }
            }
            ans
        })
        .collect()
}
