use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

pub type Point = (isize, isize, isize);

fn parse_point(input: &str) -> IResult<&str, Point> {
    (
        complete::isize,
        tag(","),
        complete::isize,
        tag(","),
        complete::isize,
    )
        .map(|(a, _, b, _, c)| (a, b, c))
        .parse(input)
}

pub fn parse_scanner(input: &str) -> IResult<&str, Vec<Point>> {
    preceded(
        (tag("--- scanner "), digit1, tag(" ---\n")),
        separated_list1(line_ending, parse_point),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<Point>> {
    all_consuming(separated_list1((line_ending, line_ending), parse_scanner))
        .parse(input)
        .unwrap()
        .1
}
