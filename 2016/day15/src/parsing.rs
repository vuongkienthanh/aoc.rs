use aoc_helper::nom::parse_number;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::line_ending, multi::separated_list1,
};

#[derive(Debug)]
pub struct Item {
    pub modulus: usize,
    pub rem: usize,
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        tag("Disc #"),
        parse_number,
        tag(" has "),
        parse_number,
        tag(" positions; at time=0, it is at position "),
        parse_number,
        tag("."),
    )
        .map(|(_, a, _, b, _, c, _)| Item {
            modulus: b,
            rem: b - ((a + c) % b),
        })
        .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(line_ending, parse_line).parse(input)
}
