use crate::min_keypresses;
pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let keypresses = min_keypresses(line, 2);
            let code = line[0..3].parse::<usize>().unwrap();
            code * keypresses
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"029A
980A
179A
379A
456A"#;
        assert_eq!(process(input), 126384);
    }
}
