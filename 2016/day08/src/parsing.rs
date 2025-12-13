use aoc_helper::nom::parse_number;
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::line_ending,
    multi::separated_list1,
};
#[derive(Debug)]
pub enum Action {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

fn parse_line(input: &str) -> IResult<&str, Action> {
    alt((
        (tag("rect "), parse_number, tag("x"), parse_number).map(|(_, a, _, b)| Action::Rect(a, b)),
        (
            tag("rotate row y="),
            parse_number,
            tag(" by "),
            parse_number,
        )
            .map(|(_, a, _, b)| Action::RotateRow(a, b)),
        (
            tag("rotate column x="),
            parse_number,
            tag(" by "),
            parse_number,
        )
            .map(|(_, a, _, b)| Action::RotateCol(a, b)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Action>> {
    separated_list1(line_ending, parse_line).parse(input)
}
