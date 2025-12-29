#[allow(unused_imports)]
// use aoc_helper::nom::parse_signed_usize;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::{count, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
};

pub type Square = Vec<Vec<char>>;

fn parse_square_2(input: &str) -> IResult<&str, Square> {
    separated_list1(tag("/"), count(complete::anychar, 2)).parse(input)
}
fn parse_square_3(input: &str) -> IResult<&str, Square> {
    separated_list1(tag("/"), count(complete::anychar, 3)).parse(input)
}
fn parse_square_4(input: &str) -> IResult<&str, Square> {
    separated_list1(tag("/"), count(complete::anychar, 4)).parse(input)
}

fn parse_line_1(input: &str) -> IResult<&str, (Square, Square)> {
    separated_pair(parse_square_2, tag(" => "), parse_square_3).parse(input)
}
fn parse_line_2(input: &str) -> IResult<&str, (Square, Square)> {
    separated_pair(parse_square_3, tag(" => "), parse_square_4).parse(input)
}
fn parse_line(input: &str) -> IResult<&str, (Square, Square)> {
    alt((parse_line_1, parse_line_2)).parse(input)
}

pub fn parse_input(input: &str) -> Vec<(Square, Square)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
