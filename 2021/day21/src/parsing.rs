use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    sequence::{preceded, separated_pair},
};

fn parse_p1(input: &str) -> IResult<&str, usize> {
    preceded(tag("Player 1 starting position: "), complete::usize).parse(input)
}
fn parse_p2(input: &str) -> IResult<&str, usize> {
    preceded(tag("Player 2 starting position: "), complete::usize).parse(input)
}

pub fn parse_input(input: &str) -> (usize, usize) {
    all_consuming(separated_pair(parse_p1, line_ending, parse_p2))
        .parse(input)
        .unwrap()
        .1
}
