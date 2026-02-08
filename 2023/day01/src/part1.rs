pub fn process(_input: &str) -> usize {
    fn is_number(c: char) -> Option<u32> {
        match c {
            '1'..='9' => c.to_digit(10),
            _ => None,
        }
    }
    _input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(is_number).unwrap();
            let last = line.chars().rev().find_map(is_number).unwrap();
            first * 10 + last
        })
        .sum::<u32>() as usize
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(process(input), 142);
    }
}
