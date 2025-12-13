use aoc_helper::nom::parse_number;
use nom::{
    IResult, Parser,
    character::complete::{line_ending, space0},
    multi::separated_list1,
    sequence::preceded,
};

type Item = (usize, usize, usize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        preceded(space0, parse_number),
        preceded(space0, parse_number),
        preceded(space0, parse_number),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(line_ending, parse_line).parse(input)
}
