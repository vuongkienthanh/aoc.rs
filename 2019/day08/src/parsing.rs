use nom::{IResult, Parser, character::complete::anychar, combinator::all_consuming, multi::many1};

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    many1(anychar.map(|x: char| x.to_digit(10).unwrap())).parse(input)
}

pub fn parse_input(input: &str) -> Vec<u32> {
    all_consuming(parse_line).parse(input).unwrap().1
}
