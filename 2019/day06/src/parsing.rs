use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alphanumeric1, tag(")"), alphanumeric1).parse(input)
}

pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
