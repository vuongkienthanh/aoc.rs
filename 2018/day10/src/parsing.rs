use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, space0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

pub type Point = (isize, isize);

fn parse_inner_pair(input: &str) -> IResult<&str, Point> {
    separated_pair(complete::isize, (tag(","), space0), complete::isize).parse(input)
}

fn parse_pair(input: &str) -> IResult<&str, Point> {
    delimited((tag("<"), space0), parse_inner_pair, (space0, tag(">"))).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    (
        preceded(tag("position="), parse_pair),
        preceded(tag(" velocity="), parse_pair),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> (Vec<Point>, Vec<Point>) {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
        .into_iter()
        .fold((vec![], vec![]), |(mut p, mut v), ele| {
            p.push(ele.0);
            v.push(ele.1);
            (p, v)
        })
}
