use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1,
    combinator::map_res,
};

pub fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

pub fn parse_integer(input: &str) -> IResult<&str, isize> {
    alt((
        (tag("+"), digit1).map(|(_, i): (_, &str)| i.parse::<isize>().unwrap()),
        (tag("-"), digit1).map(|(_, i): (_, &str)| -i.parse::<isize>().unwrap()),
        map_res(digit1, str::parse),
    ))
    .parse(input)
}
