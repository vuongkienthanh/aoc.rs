pub mod part1;
pub mod part2;

fn evolve(n: usize) -> usize {
    let a = ((n << 6) ^ n) & 16777215;
    let b = ((a >> 5) ^ a) & 16777215;
    ((b << 11) ^ b) & 16777215
}
fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}
fn prune(secret: usize) -> usize {
    secret & 16777215
}

#[cfg(test)]
mod test {
    use crate::{evolve, mix, prune};

    #[test]
    #[ignore = "reason"]
    fn test_evolve() {
        let rows = r#"123
15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254"#;
        for v in rows
            .lines()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .windows(2)
        {
            assert_eq!(evolve(v[0]), v[1]);
        }
    }
    #[test]
    #[ignore = "reason"]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }
    #[test]
    #[ignore = "reason"]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }
}
