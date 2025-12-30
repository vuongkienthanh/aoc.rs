use crate::parsing::{Direction, Instruction, State, parse_input};
use std::collections::VecDeque;

pub fn process(_input: &str) -> usize {
    let ((start_state, steps), blueprint) = parse_input(_input);
    let mut turing_machine = Machine::new(start_state);

    for _ in 0..steps {
        turing_machine.run_blueprint_once(&blueprint);
    }
    turing_machine.data.into_iter().sum()
}

struct Machine {
    idx: usize,
    state: usize,
    data: VecDeque<usize>,
}
impl Machine {
    fn new(state: usize) -> Self {
        Self {
            idx: 0,
            state,
            data: VecDeque::from([0]),
        }
    }
    fn turn_left(&mut self) {
        if self.idx == 0 {
            self.data.push_front(0);
        } else {
            self.idx -= 1
        }
    }
    fn turn_right(&mut self) {
        if self.idx == self.data.len() - 1 {
            self.data.push_back(0);
        }
        self.idx += 1
    }
    fn turn(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.turn_left(),
            Direction::Right => self.turn_right(),
        }
    }
    fn run_instruction(&mut self, instruction: &Instruction) {
        self.data[self.idx] = instruction.write;
        self.turn(&instruction.mov);
        self.state = instruction.to;
    }
    fn run_blueprint_once(&mut self, blueprint: &[State]) {
        let state = &blueprint[self.state];
        let current = self.data[self.idx];
        if current == 0 {
            self.run_instruction(&state.if0);
        } else {
            self.run_instruction(&state.if1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn fixture() -> &'static str {
        r#"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A."#
    }
    #[rstest]
    fn test_process_(fixture: &str) {
        assert_eq!(process(fixture), 3);
    }
}
