use fxhash::FxHashMap as Map;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse_line(input: &str) -> IResult<&str, (usize, Vec<usize>)> {
    separated_pair(
        complete::usize,
        tag(" <-> "),
        separated_list1(tag(", "), complete::usize),
    )
    .parse(input)
}

fn parse_map(input: &str) -> IResult<&str, Map<usize, Vec<usize>>> {
    separated_list1(line_ending, parse_line)
        .map(|v| {
            v.into_iter().fold(Map::default(), |mut acc, (i, v)| {
                acc.insert(i, v);
                acc
            })
        })
        .parse(input)
}

pub fn parse_input(input: &str) -> Map<usize, Vec<usize>> {
    all_consuming(parse_map).parse(input).unwrap().1
}
