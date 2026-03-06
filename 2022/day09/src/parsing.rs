use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_line(input: &str) -> IResult<&str, ((isize, isize), isize)> {
    alt((
        (tag("U ").map(|_| (0, 1)), complete::isize),
        (tag("D ").map(|_| (0, -1)), complete::isize),
        (tag("L ").map(|_| (-1, 0)), complete::isize),
        (tag("R ").map(|_| (1, 0)), complete::isize),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<((isize, isize), isize)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
