use aoc_helper::nom::parse_number;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, line_ending},
    multi::separated_list1,
};

#[derive(Debug)]
pub enum Item {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    MovePosition(usize, usize),
    Reverse(usize, usize),
    RotateRight(usize),
    RotateLeft(usize),
    RotateBased(char),
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((
        (
            tag("swap position "),
            parse_number,
            tag(" with position "),
            parse_number,
        )
            .map(|(_, a, _, b)| Item::SwapPosition(a, b)),
        (tag("swap letter "), anychar, tag(" with letter "), anychar)
            .map(|(_, a, _, b)| Item::SwapLetter(a, b)),
        (
            tag("move position "),
            parse_number,
            tag(" to position "),
            parse_number,
        )
            .map(|(_, a, _, b)| Item::MovePosition(a, b)),
        (
            tag("reverse positions "),
            parse_number,
            tag(" through "),
            parse_number,
        )
            .map(|(_, a, _, b)| Item::Reverse(a, b)),
        (
            tag("rotate right "),
            parse_number,
            alt((tag(" steps"), tag(" step"))),
        )
            .map(|(_, a, _)| Item::RotateRight(a)),
        (
            tag("rotate left "),
            parse_number,
            alt((tag(" steps"), tag(" step"))),
        )
            .map(|(_, a, _)| Item::RotateLeft(a)),
        (
            tag("rotate left "),
            parse_number,
            alt((tag(" steps"), tag(" step"))),
        )
            .map(|(_, a, _)| Item::RotateLeft(a)),
        (tag("rotate based on position of letter "), anychar).map(|(_, a)| Item::RotateBased(a)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(line_ending, parse_line).parse(input)
}
