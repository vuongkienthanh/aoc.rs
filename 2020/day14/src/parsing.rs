use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while},
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

fn parse_mask(input: &str) -> IResult<&str, &str> {
    preceded(tag("mask = "), take_while(|x| x != '\n')).parse(input)
}
fn parse_mem(input: &str) -> IResult<&str, (u64, u64)> {
    (tag("mem["), complete::u64, tag("] = "), complete::u64)
        .map(|(_, a, _, b)| (a, b))
        .parse(input)
}
fn parse_group(input: &str) -> IResult<&str, (&str, Vec<(u64, u64)>)> {
    separated_pair(
        parse_mask,
        line_ending,
        separated_list1(line_ending, parse_mem),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<(&str, Vec<(u64, u64)>)> {
    all_consuming(separated_list1(line_ending, parse_group))
        .parse(input)
        .unwrap()
        .1
}
