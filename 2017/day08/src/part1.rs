use crate::parsing::{Cond, Op, parse_input};
use fxhash::FxHashMap as Map;

pub fn process(_input: &str) -> isize {
    let input = parse_input(_input);
    let mut registers: Map<&str, isize> = Map::default();

    for (op, cond) in input {
        if match cond {
            Cond::GT(a, b) => *registers.entry(a).or_default() > b,
            Cond::GE(a, b) => *registers.entry(a).or_default() >= b,
            Cond::LT(a, b) => *registers.entry(a).or_default() < b,
            Cond::LE(a, b) => *registers.entry(a).or_default() <= b,
            Cond::EQ(a, b) => *registers.entry(a).or_default() == b,
            Cond::NEQ(a, b) => *registers.entry(a).or_default() != b,
        } {
            match op {
                Op::Inc(a, b) => *registers.entry(a).or_default() += b,
                Op::Dec(a, b) => *registers.entry(a).or_default() -= b,
            }
        }
    }

    registers.into_values().max().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 1);
    }
}
