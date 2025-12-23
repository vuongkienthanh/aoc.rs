use crate::Computer;
use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);
    let mut computer: Computer = Computer::new(input);
    computer.run_loop();
    computer.registers[0] as usize
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
