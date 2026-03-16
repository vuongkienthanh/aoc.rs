use crate::naive_surface;
use crate::parsing::parse_input;
use std::collections::BTreeSet;

pub fn process(_input: &str) -> usize {
    let input = BTreeSet::from_iter(parse_input(_input).into_iter().map(|(x, y, z)| (z, y, x)));
    naive_surface(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 64);
    }
}
