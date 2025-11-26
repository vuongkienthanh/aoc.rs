use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
};

type Item = (u32, u32);

fn parse_addr(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    let (input, (a, b)) = separated_pair(parse_addr, tag("-"), parse_addr).parse(input)?;
    assert!(a < b);
    Ok((input, (a, b)))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(line_ending, parse_line).parse(input)
}
