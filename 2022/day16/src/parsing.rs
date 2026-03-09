use fxhash::FxHashMap;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
};

fn parse_line(input: &str) -> IResult<&str, (&str, usize, Vec<&str>)> {
    (
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), complete::usize),
        preceded(
            (
                alt((tag("; tunnels lead to "), tag("; tunnel leads to "))),
                alt((tag("valves "), tag("valve "))),
            ),
            separated_list1(tag(", "), alpha1),
        ),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> FxHashMap<&str, (usize, Vec<&str>)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
        .into_iter()
        .map(|(from, rate, to)| (from, (rate, to)))
        .collect()
}
