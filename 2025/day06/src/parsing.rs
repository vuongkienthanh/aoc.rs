use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, char, line_ending, space0, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};

#[derive(Debug, Eq, PartialEq)]
pub enum Item {
    Add,
    Mul,
}

fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(space0, separated_list1(space1, complete::usize), space0).parse(input)
}
fn parse_ops(input: &str) -> IResult<&str, Vec<Item>> {
    delimited(
        space0,
        separated_list1(
            space1,
            alt((char('+').map(|_| Item::Add), char('*').map(|_| Item::Mul))),
        ),
        space0,
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Item>) {
    all_consuming(separated_pair(
        separated_list1(line_ending, parse_line),
        line_ending,
        parse_ops,
    ))
    .parse(input)
    .unwrap()
    .1
}
