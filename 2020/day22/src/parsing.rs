use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};
use std::collections::VecDeque;

fn parse_player1(input: &str) -> IResult<&str, VecDeque<usize>> {
    preceded(
        tag("Player 1:\n"),
        separated_list1(line_ending, complete::usize),
    )
    .map(|v| VecDeque::from(v))
    .parse(input)
}
fn parse_player2(input: &str) -> IResult<&str, VecDeque<usize>> {
    preceded(
        tag("\n\nPlayer 2:\n"),
        separated_list1(line_ending, complete::usize),
    )
    .map(|v| VecDeque::from(v))
    .parse(input)
}

pub fn parse_input(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    all_consuming((parse_player1, parse_player2))
        .parse(input)
        .unwrap()
        .1
}
