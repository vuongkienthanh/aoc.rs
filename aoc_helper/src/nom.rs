use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending},
    combinator::map_res,
    sequence::{preceded, terminated},
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

pub enum Sign {
    Positive,
    Negative,
}

pub fn line(input: &str) -> IResult<&str, &str> {
    terminated(take_until("\n"), line_ending).parse(input)
}

pub fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

pub fn parse_integer(input: &str) -> IResult<&str, (Sign, usize)> {
    alt((
        preceded(tag("+"), digit1).map(|x: &str| (Sign::Positive, x.parse().unwrap())),
        preceded(tag("-"), digit1).map(|x: &str| (Sign::Negative, x.parse().unwrap())),
        digit1.map(|x: &str| (Sign::Positive, x.parse().unwrap())),
    ))
    .parse(input)
}

pub fn parse_isize(input: &str) -> IResult<&str, isize> {
    alt((
        preceded(tag("+"), digit1).map(|x: &str| x.parse::<isize>().unwrap()),
        preceded(tag("-"), digit1).map(|x: &str| -(x.parse::<isize>().unwrap())),
        digit1.map(|x: &str| x.parse().unwrap()),
    ))
    .parse(input)
}
