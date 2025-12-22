use crate::TICKER_TAPE;
use crate::parsing::parse_input;

const CATS: usize = 1;
const TREES: usize = 7;
const POMERANIANS: usize = 3;
const GOLDFISH: usize = 6;
const REST: [usize; 6] = [0, 2, 4, 5, 8, 9];

pub fn process(_input: &str) -> usize {
    let mut aunt_list = parse_input(_input);

    while let Some((aunt, prop)) = aunt_list.pop() {
        if [CATS, TREES].into_iter().all(|i| match prop[i] {
            None => true,
            Some(n) => n > TICKER_TAPE[i],
        }) && [POMERANIANS, GOLDFISH].into_iter().all(|i| match prop[i] {
            None => true,
            Some(n) => n < TICKER_TAPE[i],
        }) && REST.into_iter().all(|i| match prop[i] {
            None => true,
            Some(n) => n == TICKER_TAPE[i],
        }) {
            return aunt;
        }
    }

    panic!("should have an answer")
}
