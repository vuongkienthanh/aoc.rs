use nom::{
    IResult, Parser,
    bytes::complete::{tag, take, take_while},
    character::complete::{alpha1, line_ending},
    combinator::recognize,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};
use std::collections::HashMap;

pub type Map<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_ele(input: &str) -> IResult<&str, &str> {
    recognize((take(1usize), take_while(|x: char| x.is_ascii_lowercase()))).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(parse_ele, tag(" => "), alpha1).parse(input)
}

pub fn parse_map<'a>(input: &'a str) -> IResult<&'a str, Map<'a>> {
    let (input, lines) = separated_list1(line_ending, parse_line).parse(input)?;
    let map = lines
        .into_iter()
        .fold(Map::new(), |mut acc, (left, right)| {
            acc.entry(left).or_default().push(right);
            acc
        });
    Ok((input, map))
}

pub fn parse_molecules(input: &str) -> IResult<&str, Vec<&str>> {
    many1(parse_ele).parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> IResult<&'a str, (Map<'a>, Vec<&'a str>)> {
    (parse_map, line_ending, line_ending, parse_molecules)
        .map(|(m, _, _, v)| (m, v))
        .parse(input)
}
