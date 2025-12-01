use aoc_helper::nom::parse_isize;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending},
    multi::separated_list1,
    sequence::preceded,
};

fn parse_line(input: &str) -> IResult<&str, isize> {
    alt((
        preceded(char('L'), parse_isize).map(|x| -x),
        preceded(char('R'), parse_isize),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<isize>> {
    separated_list1(line_ending, parse_line).parse(input)
}
