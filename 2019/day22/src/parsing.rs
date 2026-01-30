use aoc_helper::nom::{Sign, parse_signed_usize};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

#[derive(Debug)]
pub enum Item {
    Reverse,
    Cut(Sign, usize),
    Deal(usize),
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((
        tag("deal into new stack").map(|_| Item::Reverse),
        preceded(tag("cut "), parse_signed_usize).map(|(s, u)| Item::Cut(s, u)),
        preceded(tag("deal with increment "), complete::usize).map(Item::Deal),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
