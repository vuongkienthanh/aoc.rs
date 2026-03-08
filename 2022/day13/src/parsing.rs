use crate::Value;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
};

pub fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        complete::u8.map(Value::Int),
        delimited(tag("["), separated_list0(tag(","), parse_value), tag("]")).map(Value::List),
    ))
    .parse(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Value, Value)> {
    separated_pair(parse_value, line_ending, parse_value).parse(input)
}

pub fn parse_input(input: &str) -> Vec<(Value, Value)> {
    all_consuming(separated_list1((line_ending, line_ending), parse_pair))
        .parse(input)
        .unwrap()
        .1
}
