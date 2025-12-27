use crate::parsing::parse_input;
use crate::run;

pub fn process(_input: &str) -> String {
    let input = parse_input(_input);
    let mut dancers: Vec<char> = ('a'..='p').collect();
    run(&mut dancers, &input);
    dancers.into_iter().collect()
}
