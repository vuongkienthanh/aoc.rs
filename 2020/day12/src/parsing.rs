#[allow(unused_imports)]
// use aoc_helper::nom::parse_signed_usize;
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
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((
        preceded(tag("N"), complete::isize).map(Item::N),
        preceded(tag("S"), complete::isize).map(Item::S),
        preceded(tag("E"), complete::isize).map(Item::E),
        preceded(tag("W"), complete::isize).map(Item::W),
        preceded(tag("L"), complete::isize).map(Item::L),
        preceded(tag("R"), complete::isize).map(Item::R),
        preceded(tag("F"), complete::isize).map(Item::F),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
