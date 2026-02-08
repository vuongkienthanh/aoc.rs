pub fn process(_input: &str) -> usize {
    fn is_number(c: char, i: usize, line: &str) -> Option<u32> {
        match c {
            '1'..='9' => c.to_digit(10),
            'o' if line.get(i..i + 3) == Some("one") => Some(1),
            't' if line.get(i..i + 3) == Some("two") => Some(2),
            't' if line.get(i..i + 5) == Some("three") => Some(3),
            'f' if line.get(i..i + 4) == Some("four") => Some(4),
            'f' if line.get(i..i + 4) == Some("five") => Some(5),
            's' if line.get(i..i + 3) == Some("six") => Some(6),
            's' if line.get(i..i + 5) == Some("seven") => Some(7),
            'e' if line.get(i..i + 5) == Some("eight") => Some(8),
            'n' if line.get(i..i + 4) == Some("nine") => Some(9),
            _ => None,
        }
    }
    _input
        .lines()
        .map(|line| {
            let first = line
                .char_indices()
                .find_map(|(i, c)| is_number(c, i, line))
                .unwrap();
            let last = line
                .char_indices()
                .rev()
                .find_map(|(i, c)| is_number(c, i, line))
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>() as usize
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(process(input), 281);
    }
}
