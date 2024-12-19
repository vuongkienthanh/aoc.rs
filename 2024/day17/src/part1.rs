use crate::{Computer, Instruction};
pub fn process(_input: &str) -> String {
    let (regs, prog) = _input.split_once("\n\n").unwrap();
    let mut regs = regs
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap());
    let mut computer = Computer {
        register_a: regs.next().unwrap(),
        register_b: regs.next().unwrap(),
        register_c: regs.next().unwrap(),
        pointer: 0,
        output: vec![],
    };
    let program = prog
        .split_once(": ")
        .unwrap()
        .1
        .chars()
        .step_by(2)
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    while computer.pointer < program.len() {
        Instruction::run(
            program[computer.pointer],
            program[computer.pointer + 1],
            &mut computer,
        );
    }

    computer
        .output
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        assert_eq!(process(input), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
