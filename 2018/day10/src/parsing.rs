#[allow(unused_imports)]
// use aoc_helper::nom::parse_signed_usize;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

type Point = (isize, isize);
type Item = (Point, Point);

fn parse_inner_pair(input: &str) -> IResult<&str, Point> {
    separated_pair(complete::isize, (tag(","), space0), complete::isize).parse(input)
}

fn parse_pair(input: &str) -> IResult<&str, Point> {
    delimited((tag("<"), space0), parse_inner_pair, (space0, tag(">"))).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        preceded(tag("position="), parse_pair),
        preceded(tag(" velocity="), parse_pair),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
