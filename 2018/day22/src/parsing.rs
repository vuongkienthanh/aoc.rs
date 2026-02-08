use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    sequence::{preceded, separated_pair},
};

fn parse_depth(input: &str) -> IResult<&str, usize> {
    preceded(tag("depth: "), complete::usize).parse(input)
}

fn parse_target(input: &str) -> IResult<&str, (usize, usize)> {
    preceded(
        tag("target: "),
        separated_pair(complete::usize, tag(","), complete::usize),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> (usize, (usize, usize)) {
    all_consuming(separated_pair(parse_depth, line_ending, parse_target))
        .parse(input)
        .unwrap()
        .1
}