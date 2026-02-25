use aoc_helper::cube::Cube;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

fn parse_line(input: &str) -> IResult<&str, (bool, Cube)> {
    (
        alt((tag("on ").map(|_| true), tag("off ").map(|_| false))),
        (
            separated_pair(
                preceded(tag("x="), complete::isize),
                tag(".."),
                complete::isize,
            ),
            separated_pair(
                preceded(tag(",y="), complete::isize),
                tag(".."),
                complete::isize,
            ),
            separated_pair(
                preceded(tag(",z="), complete::isize),
                tag(".."),
                complete::isize,
            ),
        ),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<(bool, Cube)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
