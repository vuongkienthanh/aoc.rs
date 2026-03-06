use crate::follow;
use crate::parsing::parse_input;
use fxhash::FxHashSet;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut tail_map = FxHashSet::default();
    let mut rope = [(0, 0); 10];
    tail_map.insert(rope[9]);
    for ((x, y), i) in input {
        for _ in 0..i {
            rope[0] = (rope[0].0 + x, rope[0].1 + y);
            rope[1] = follow(rope[0], rope[1]);
            for idx in 1..rope.len() - 1 {
                let left = rope[idx];
                let right = rope[idx + 1];
                let new_right = follow(left, right);
                rope[idx + 1] = new_right;
            }
            tail_map.insert(rope[9]);
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
        r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 36);
    }
}
