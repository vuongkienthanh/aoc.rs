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

#[derive(Debug)]
pub enum Item {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_spin(input: &str) -> IResult<&str, Item> {
    preceded(complete::char('s'), complete::usize)
        .map(|x| Item::Spin(x))
        .parse(input)
}
fn parse_exchange(input: &str) -> IResult<&str, Item> {
    (
        complete::char('x'),
        complete::usize,
        complete::char('/'),
        complete::usize,
    )
        .map(|(_, a, _, b)| Item::Exchange(a, b))
        .parse(input)
}
fn parse_partner(input: &str) -> IResult<&str, Item> {
    (
        complete::char('p'),
        complete::anychar,
        complete::char('/'),
        complete::anychar,
    )
        .map(|(_, a, _, b)| Item::Partner(a, b))
        .parse(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((parse_spin, parse_exchange, parse_partner)).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(complete::char(','), parse_item))
        .parse(input)
        .unwrap()
        .1
}
