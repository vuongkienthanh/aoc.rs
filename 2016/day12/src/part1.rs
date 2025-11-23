use crate::parsing::{Item, parse_input};
use aoc_helper::assembly::Computer;

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    // println!("{input:?}");
    // println!("{_rest:?}");
    assert!(_rest.is_empty());
    let mut computer: Computer<4, Item> = Computer::new(input);
    computer.run();
    computer.registers[0]
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"#
    }
    #[rstest]
    fn test_process(fixture: &str) {
        assert_eq!(process(fixture), 42);
    }
}
