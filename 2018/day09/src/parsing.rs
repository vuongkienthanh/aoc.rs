#[allow(unused_imports)]
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

type Item = (usize, usize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        complete::usize,
        tag(" players; last marble is worth "),
        complete::usize,
        tag(" points"),
    )
        .map(|(a, _, b, _)| (a, b))
        .parse(input)
}

pub fn parse_input(input: &str) -> Item {
    all_consuming(parse_line).parse(input).unwrap().1
}
