use super::{find_token, parse};
pub fn process(_input: &str) -> isize {
    let (_, v) = parse(_input).unwrap();
    v.into_iter()
        .filter_map(|(a, b, p)| {
            let p = (10_000_000_000_000 + p.0, 10_000_000_000_000 + p.1);
            find_token(a, b, p)
        })
        .sum()
}
