use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

type Point = (usize, usize);

fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(complete::usize, tag(","), complete::usize).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_point).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<Point>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}