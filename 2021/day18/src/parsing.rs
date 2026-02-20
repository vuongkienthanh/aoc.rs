use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item {
    Literal(usize),
    Number(Number),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Number {
    pub left: Box<Item>,
    pub right: Box<Item>,
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        complete::usize.map(Item::Literal),
        parse_number.map(Item::Number),
    ))
    .parse(input)
}

pub fn parse_number(input: &str) -> IResult<&str, Number> {
    delimited(
        tag("["),
        separated_pair(parse_item, tag(","), parse_item),
        tag("]"),
    )
    .map(|(a, b)| Number {
        left: Box::new(a),
        right: Box::new(b),
    })
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Number> {
    all_consuming(separated_list1(line_ending, parse_number))
        .parse(input)
        .unwrap()
        .1
}


