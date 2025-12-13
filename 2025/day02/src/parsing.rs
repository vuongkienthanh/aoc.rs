use nom::{
    IResult, Parser,
    character::complete::{char, digit1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse_item(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(digit1, char('-'), digit1).parse(input)
}

pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    all_consuming(separated_list1(char(','), parse_item))
        .parse(input)
        .unwrap()
        .1
}
