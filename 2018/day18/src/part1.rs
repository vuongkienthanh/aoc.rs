use crate::{next, parse, resource_value};

pub fn process(_input: &str) -> usize {
    let mut outskirts = parse(_input);
    for _ in 0..10 {
        outskirts = next(outskirts);
    }
    resource_value(outskirts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 1147);
    }
}
