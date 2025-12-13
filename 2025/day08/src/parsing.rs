use nom::{
    IResult, Parser,
    character::complete::{self, char, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

pub type Point = (usize, usize, usize);

fn parse_line(input: &str) -> IResult<&str, Point> {
    (
        complete::usize,
        char(','),
        complete::usize,
        char(','),
        complete::usize,
    )
        .map(|(a, _, b, _, c)| (a, b, c))
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Point> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
