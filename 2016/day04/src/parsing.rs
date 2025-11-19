use aoc_helper::nom::parse_number;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::recognize,
    multi::{many1, separated_list1},
    sequence::delimited,
};

type Item<'a> = (&'a str, usize, &'a str);

fn parse_encrypted_name(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((alpha1, tag("-"))))).parse(input)
}
fn parse_checksum(input: &str) -> IResult<&str, &str> {
    delimited(tag("["), alpha1, tag("]")).parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    (parse_encrypted_name, parse_number, parse_checksum).parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Vec<Item<'a>>> {
    separated_list1(line_ending, parse_line).parse(input)
}
