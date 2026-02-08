use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self},
    combinator::{all_consuming, not},
    multi::{many0, separated_list1},
    sequence::delimited,
};

#[derive(Debug)]
pub enum Item {
    Garbage(usize),
    Group(Vec<Item>),
}

fn parse_inside_garbage(input: &str) -> IResult<&str, usize> {
    many0(alt((
        (complete::char('!'), take(1usize)).map(|_| 0),
        (not(complete::char('>')), take(1usize)).map(|_| 1),
    )))
    .map(|v| v.into_iter().sum())
    .parse(input)
}
fn parse_garbage(input: &str) -> IResult<&str, Item> {
    delimited(tag("<"), parse_inside_garbage, tag(">"))
        .map(Item::Garbage)
        .parse(input)
}
fn parse_inside_group(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(tag(","), alt((parse_group, parse_garbage))).parse(input)
}
fn parse_group(input: &str) -> IResult<&str, Item> {
    alt((
        delimited(tag("{"), parse_inside_group, tag("}")).map(Item::Group),
        tag("{}").map(|_| Item::Group(Vec::with_capacity(0))),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Item {
    all_consuming(alt((parse_group, parse_garbage)))
        .parse(input)
        .unwrap()
        .1
}
