pub fn process(_input: &str) -> usize {
    todo!("part1")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"029A
980A
179A
456A
379A"#;
        assert_eq!(process(input), 126384);
    }
}
