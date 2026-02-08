use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::all_consuming,
    multi::{count, separated_list1},
};

pub enum Item {
    Bug,
    Space,
}

fn parse_line(input: &str) -> IResult<&str, Vec<Item>> {
    count(
        alt((tag("#").map(|_| Item::Bug), tag(".").map(|_| Item::Space))),
        5,
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<Item>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
