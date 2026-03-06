use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

fn parse_line(input: &str) -> IResult<&str, Option<isize>> {
    alt((
        tag("noop").map(|_| None),
        preceded(tag("addx "), complete::isize).map(|x| Some(x)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Option<isize>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
