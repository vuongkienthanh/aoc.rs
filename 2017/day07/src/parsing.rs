#[allow(unused_imports)]
// use aoc_helper::nom::parse_signed_usize;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

#[derive(Debug)]
pub enum Item<'a> {
    Name(&'a str, usize),
    Assign(&'a str, usize, Vec<&'a str>),
}

fn parse_name<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    (alpha1, delimited(tag(" ("), complete::usize, tag(")")))
        .map(|(a, b)| Item::Name(a, b))
        .parse(input)
}
fn parse_assign<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    (
        alpha1,
        delimited(tag(" ("), complete::usize, tag(")")),
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )
        .map(|(a, b, _, c)| Item::Assign(a, b, c))
        .parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    alt((parse_assign, parse_name)).parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<Item<'a>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
