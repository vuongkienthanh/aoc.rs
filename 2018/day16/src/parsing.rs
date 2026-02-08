#[allow(unused_imports)]
// use aoc_helper::nom::parse_signed_usize;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space1},
    combinator::all_consuming,
    multi::{count, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

pub type Item = [usize; 4];

fn parse_quad(input: &str) -> IResult<&str, Item> {
    separated_list1(tag(", "), complete::usize)
        .map(|v| [v[0], v[1], v[2], v[3]])
        .parse(input)
}
fn parse_line(input: &str) -> IResult<&str, Item> {
    separated_list1(tag(" "), complete::usize)
        .map(|v| [v[0], v[1], v[2], v[3]])
        .parse(input)
}

pub fn parse_block(input: &str) -> IResult<&str, (Item, Item, Item)> {
    (
        preceded(
            (tag("Before:"), space1),
            delimited(tag("["), parse_quad, tag("]")),
        ),
        line_ending,
        parse_line,
        line_ending,
        preceded(
            (tag("After: "), space1),
            delimited(tag("["), parse_quad, tag("]")),
        ),
    )
        .map(|(a, _, b, _, c)| (a, b, c))
        .parse(input)
}

fn parse_blocks(input: &str) -> IResult<&str, Vec<(Item, Item, Item)>> {
    separated_list1((line_ending, line_ending), parse_block).parse(input)
}

fn parse_test(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(line_ending, parse_line).parse(input)
}

pub fn parse_input(input: &str) -> (Vec<(Item, Item, Item)>, Vec<Item>) {
    all_consuming(separated_pair(
        parse_blocks,
        count(line_ending, 4),
        parse_test,
    ))
    .parse(input)
    .unwrap()
    .1
}
