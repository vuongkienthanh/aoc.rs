use crate::parsing::{Item, parse_input};
use crate::{Computer, OPCODE_LIST};

pub fn process(_input: &str) -> usize {
    let (blocks, _) = parse_input(_input);

    blocks
        .into_iter()
        .filter(|(before, [_, a, b, c], after)| check(*before, *after, *a, *b, *c) >= 3)
        .count()
}
fn check(before: Item, after: Item, a: usize, b: usize, c: usize) -> usize {
    OPCODE_LIST
        .iter()
        .filter(|opcode| {
            let mut comp = Computer::from_arr(before);
            comp.run(*opcode, a, b, c);
            comp.registers == after
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::parse_block;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"#
    }
    #[rstest]
    fn test_check(fixture: &str) {
        let (_, (before, [_, a, b, c], after)) = parse_block(fixture).unwrap();
        assert_eq!(check(before, after, a, b, c), 3);
    }
}
