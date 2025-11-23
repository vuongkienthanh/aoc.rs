use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1,
    combinator::map_res, sequence::preceded,
};

pub enum Sign {
    Positive,
    Negative,
}

pub fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

pub fn parse_integer(input: &str) -> IResult<&str, (Sign, usize)> {
    alt((
        preceded(tag("+"), digit1).map(|x:&str| (Sign::Positive, x.parse().unwrap())),
        preceded(tag("-"), digit1).map(|x:&str| (Sign::Negative, x.parse().unwrap())),
        digit1.map(|x:&str| (Sign::Positive, x.parse().unwrap())),
    ))
    .parse(input)
}
