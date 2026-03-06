use crate::follow;
use crate::parsing::parse_input;
use fxhash::FxHashSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut tail_map = FxHashSet::default();
    let mut rope = [(0, 0), (0, 0)];
    tail_map.insert(rope[1]);
    for ((x, y), i) in input {
        for _ in 0..i {
            rope[0] = (rope[0].0 + x, rope[0].1 + y);
            let new_tail = follow(rope[0], rope[1]);
            tail_map.insert(new_tail);
            rope[1] = new_tail;
        }
    }
    tail_map.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 13);
    }
}
