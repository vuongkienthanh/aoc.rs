pub mod part1;
pub mod part2;

fn first_requirement(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|v| (v[0] as u8 + 1) == (v[1] as u8) && (v[0] as u8 + 2) == (v[2] as u8))
}
fn second_requirement(input: &str) -> bool {
    ['i', 'o', 'l'].into_iter().all(|c| !input.contains(c))
}
fn third_requirement(input: &str) -> bool {
    let mut cs = input.chars().peekable();
    let mut pair_count = 0;
    while let Some(c1) = cs.next() {
        if let Some(c2) = cs.peek()
            && c1 == *c2
        {
            pair_count += 1;
            cs.next().unwrap();
        }
    }

    pair_count >= 2
}

fn next_char(c: char) -> (char, bool) {
    if c == 'z' {
        ('a', true)
    } else {
        ((c as u8 + 1) as char, false)
    }
}

fn next_password(password: String) -> String {
    let mut advance = true;
    let mut ret = vec![];
    for mut c in password.chars().rev() {
        if advance {
            (c, advance) = next_char(c);
            ret.push(c);
        } else {
            ret.push(c);
        }
    }

    ret.into_iter().rev().collect()
}
fn next_valid_password(mut password: String) -> String {
    loop {
        password = next_password(password);
        if first_requirement(&password)
            && second_requirement(&password)
            && third_requirement(&password)
        {
            return password;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("hijklmmn", true)]
    #[case("abbceffg", false)]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    fn test_first(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(first_requirement(input), expected);
    }
    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    fn test_second(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(second_requirement(input), expected);
    }
    #[rstest]
    #[case("abbceffg", true)]
    #[case("abbcegjk", false)]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    fn test_third(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(third_requirement(input), expected);
    }
    #[rstest]
    #[case("xx", "xy")]
    #[case("xy", "xz")]
    #[case("xz", "ya")]
    #[case("ya", "yb")]
    fn test_next_password(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(next_password(input.to_string()), expected.to_string());
    }
    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_next_valid_password(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(next_valid_password(input.to_string()), expected.to_string());
    }
}
