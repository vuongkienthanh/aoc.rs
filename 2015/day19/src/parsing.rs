use nom::{
    IResult, Parser,
    bytes::complete::{tag, take, take_while},
    character::complete::{alpha1, line_ending},
    combinator::{all_consuming, recognize},
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

fn parse_molecule(input: &str) -> IResult<&str, &str> {
    recognize((take(1usize), take_while(|x: char| x.is_ascii_lowercase()))).parse(input)
}

pub fn parse_molecules(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, ms) = alpha1(input)?;
    let (_, m) = many1(parse_molecule).parse(ms)?;
    Ok((input, m))
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(parse_molecule, tag(" => "), parse_molecules).parse(input)
}

pub fn parse_map(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(line_ending, parse_line).parse(input)
}

#[allow(clippy::type_complexity)]
pub fn parse_input(input: &str) -> (Vec<(&str, Vec<&str>)>, Vec<&str>) {
    all_consuming((parse_map, many1(line_ending), parse_molecules).map(|(m, _, v)| (m, v)))
        .parse(input)
        .unwrap()
        .1
}
