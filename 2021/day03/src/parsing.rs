use nom::{
    IResult, Parser,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_item(input: &str) -> IResult<&str, u16> {
    complete::bin_digit1
        .map(|x| u16::from_str_radix(x, 2).unwrap())
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<u16> {
    all_consuming(separated_list1(line_ending, parse_item))
        .parse(input)
        .unwrap()
        .1
}