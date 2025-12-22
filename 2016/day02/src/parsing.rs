use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

#[derive(Debug, Eq, PartialEq)]
pub enum Item {
    R,
    L,
    U,
    D,
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        char('R').map(|_| Item::R),
        char('L').map(|_| Item::L),
        char('U').map(|_| Item::U),
        char('D').map(|_| Item::D),
    ))
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Item>> {
    many1(parse_item).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<Item>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
