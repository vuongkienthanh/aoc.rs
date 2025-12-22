use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
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
        (tag("rect "), complete::usize, tag("x"), complete::usize)
            .map(|(_, a, _, b)| Action::Rect(a, b)),
        (
            tag("rotate row y="),
            complete::usize,
            tag(" by "),
            complete::usize,
        )
            .map(|(_, a, _, b)| Action::RotateRow(a, b)),
        (
            tag("rotate column x="),
            complete::usize,
            tag(" by "),
            complete::usize,
        )
            .map(|(_, a, _, b)| Action::RotateCol(a, b)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Action> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
