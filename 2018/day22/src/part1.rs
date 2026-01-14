use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let (depth, target) = parse_input(_input);
    let mut ero1: Vec<usize> = vec![];
    let mut ans = 0;

    for y in 0..target.1 + 1 {
        let mut ero2 = vec![];
        for x in 0..target.0 + 1 {
            let geo = match (x, y) {
                (0, 0) => 0,
                loc if loc == target => 0,
                (x, 0) => x * 16807,
                (0, y) => y * 48271,
                (x, _) => ero2.last().unwrap() * ero1[x],
            };
            ero2.push((geo + depth) % 20183);
            ans += ero2.last().unwrap() % 3;
        }
        ero1 = ero2;
    }
    ans
}
