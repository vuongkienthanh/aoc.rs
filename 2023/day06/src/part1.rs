pub fn process(_input: &str) -> usize {
    todo!("part1")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(process(input), 288);
    }
}
