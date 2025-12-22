use crate::parsing::parse_input;
use std::cmp::Ordering;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    input
        .into_iter()
        .filter_map(|(name, i, checksum)| is_real_room(name, checksum).then_some(i))
        .sum()
}

fn is_real_room(name: &str, checksum: &str) -> bool {
    let mut map = [0; 26];
    for c in name.bytes() {
        match c {
            b'-' => (),
            b'a'..=b'z' => *map.get_mut((c - b'a') as usize).unwrap() += 1,
            _ => panic!("should be 'a'..='z' and '-'"),
        }
    }
    let mut vec: Vec<(char, usize)> = ('a'..='z').zip(map).collect();
    vec.sort_unstable_by(|a, b| match b.1.cmp(&a.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        o => o,
    });
    vec.into_iter().map(|(k, _)| k).take(5).collect::<String>() == checksum
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("aaaaa-bbb-z-y-x-", "abxyz", true)]
    #[case("a-b-c-d-e-f-g-h-", "abcde", true)]
    #[case("not-a-real-room", "oarel", true)]
    #[case("totally-real-room", "decoy", false)]
    fn test_is_real_room(#[case] name: &str, #[case] checksum: &str, #[case] expected: bool) {
        assert_eq!(is_real_room(name, checksum), expected);
    }
}
