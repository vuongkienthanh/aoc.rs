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
    F(usize),
    U(usize),
    D(usize),
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        preceded(tag("forward "), complete::usize).map(Item::F),
        preceded(tag("up "), complete::usize).map(Item::U),
        preceded(tag("down "), complete::usize).map(Item::D),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_item))
        .parse(input)
        .unwrap()
        .1
}