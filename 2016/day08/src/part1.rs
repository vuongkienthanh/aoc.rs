use crate::parsing::parse_input;
use crate::{COLS, ROWS, swipe};

pub fn process(_input: &str) -> usize {
    let (_rest, input) = parse_input(_input).unwrap();
    assert!(_rest.is_empty());
    // println!("{input:?}");

    let mut screen = [[0; COLS]; ROWS];

    swipe(&mut screen, input);
    screen
        .into_iter()
        .map(|x| x.into_iter().sum::<usize>())
        .sum()
}
