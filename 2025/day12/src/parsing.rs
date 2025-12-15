#[allow(unused_imports)]
use aoc_helper::nom::line;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, char, digit1, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

pub type Size = (usize, usize);
pub type Item = (Size, [usize; 6]);

fn parse_block(input: &str) -> IResult<&str, ()> {
    (
        digit1,
        char(':'),
        line_ending,
        line,
        line,
        line,
        line_ending,
    )
        .map(|_| ())
        .parse(input)
}
fn parse_blocks(input: &str) -> IResult<&str, ()> {
    separated_list1(line_ending, parse_block)
        .map(|_| ())
        .parse(input)
}
fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        terminated(
            separated_pair(complete::usize, char('x'), complete::usize),
            tag(": "),
        ),
        separated_list1(space1, complete::usize).map(|v| [v[0], v[1], v[2], v[3], v[4], v[5]]),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(preceded(
        parse_blocks,
        separated_list1(line_ending, parse_line),
    ))
    .parse(input)
    .unwrap()
    .1
}
