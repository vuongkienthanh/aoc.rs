use crate::build_grid;

pub fn process(_input: &str) -> usize {
    let data = build_grid(_input);
    data.into_iter()
        .map(|row| row.into_iter().sum::<usize>())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"flqrgnkx"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 8108);
    }
}
