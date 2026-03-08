use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

pub type Point = (isize, isize);

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    separated_pair(
        separated_pair(
            preceded(tag("Sensor at x="), complete::isize),
            tag(", y="),
            complete::isize,
        ),
        tag(": "),
        separated_pair(
            preceded(tag("closest beacon is at x="), complete::isize),
            tag(", y="),
            complete::isize,
        ),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<(Point, Point)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
