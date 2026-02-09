use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Item {
    nop(isize),
    acc(isize),
    jmp(isize),
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((
        preceded(tag("nop "), complete::isize).map(Item::nop),
        preceded(tag("acc "), complete::isize).map(Item::acc),
        preceded(tag("jmp "), complete::isize).map(Item::jmp),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
