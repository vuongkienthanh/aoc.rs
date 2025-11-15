use nom::{character::complete::digit1, combinator::map_res, IResult, Parser};

pub fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}
