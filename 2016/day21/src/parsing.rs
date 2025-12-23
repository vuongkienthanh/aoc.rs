use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, line_ending},
    combinator::all_consuming,
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
            complete::usize,
            tag(" with position "),
            complete::usize,
        )
            .map(|(_, a, _, b)| Item::SwapPosition(a, b)),
        (tag("swap letter "), anychar, tag(" with letter "), anychar)
            .map(|(_, a, _, b)| Item::SwapLetter(a, b)),
        (
            tag("move position "),
            complete::usize,
            tag(" to position "),
            complete::usize,
        )
            .map(|(_, a, _, b)| Item::MovePosition(a, b)),
        (
            tag("reverse positions "),
            complete::usize,
            tag(" through "),
            complete::usize,
        )
            .map(|(_, a, _, b)| Item::Reverse(a, b)),
        (
            tag("rotate right "),
            complete::usize,
            alt((tag(" steps"), tag(" step"))),
        )
            .map(|(_, a, _)| Item::RotateRight(a)),
        (
            tag("rotate left "),
            complete::usize,
            alt((tag(" steps"), tag(" step"))),
        )
            .map(|(_, a, _)| Item::RotateLeft(a)),
        (
            tag("rotate left "),
            complete::usize,
            alt((tag(" steps"), tag(" step"))),
        )
            .map(|(_, a, _)| Item::RotateLeft(a)),
        (tag("rotate based on position of letter "), anychar).map(|(_, a)| Item::RotateBased(a)),
    ))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
