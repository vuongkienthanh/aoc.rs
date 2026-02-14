use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, space0},
    combinator::all_consuming,
    multi::{count, separated_list1},
    sequence::preceded,
};
use std::convert::TryInto;

fn parse_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), complete::u8).parse(input)
}
fn parse_board(input: &str) -> IResult<&str, [u8; 25]> {
    separated_list1(line_ending, count(preceded(space0, complete::u8), 5))
        .map(|v| {
            v.into_iter()
                .flatten()
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .parse(input)
}
fn parse_boards(input: &str) -> IResult<&str, Vec<[u8; 25]>> {
    preceded(
        (line_ending, line_ending),
        separated_list1((line_ending, line_ending), parse_board),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> (Vec<u8>, Vec<[u8; 25]>) {
    all_consuming((parse_numbers, parse_boards))
        .parse(input)
        .unwrap()
        .1
}