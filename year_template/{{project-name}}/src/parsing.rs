#[allow(unused_imports)]
// use aoc_helper::nom::parse_signed_usize;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

type Item = usize;

fn parse_line(input: &str) -> IResult<&str, Item> {
    todo!()
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
