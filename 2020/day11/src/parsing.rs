use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item {
    Space,
    Empty,
    Occupied,
}
fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        tag(".").map(|_| Item::Space),
        tag("L").map(|_| Item::Empty),
        tag("#").map(|_| Item::Occupied),
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
