use fxhash::FxHashSet;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::{count, many1, separated_list1},
    sequence::{preceded, separated_pair},
};
use std::collections::VecDeque;

pub type Map = FxHashSet<Vec<usize>>;

fn parse_plants(input: &str) -> IResult<&str, VecDeque<usize>> {
    preceded(
        tag("initial state: "),
        many1(alt((
            complete::char('.').map(|_| 0),
            complete::char('#').map(|_| 1),
        ))),
    )
    .map(|mut v| {
        v[0] += 2;
        v.into_iter().collect()
    })
    .parse(input)
}

fn parse_map_line(input: &str) -> IResult<&str, Option<Vec<usize>>> {
    separated_pair(
        count(alt((complete::char('.'), complete::char('#'))), 5),
        tag(" => "),
        alt((complete::char('.'), complete::char('#'))),
    )
    .map(|(a, b)| {
        if b == '#' {
            Some(
                a.iter()
                    .enumerate()
                    .filter_map(|(i, x)| (x == &'.').then_some(i))
                    .collect(),
            )
        } else {
            None
        }
    })
    .parse(input)
}
fn parse_map(input: &str) -> IResult<&str, Map> {
    separated_list1(line_ending, parse_map_line)
        .map(|v| v.into_iter().flatten().collect())
        .parse(input)
}

pub fn parse_input(input: &str) -> (VecDeque<usize>, Map) {
    all_consuming(separated_pair(
        parse_plants,
        (line_ending, line_ending),
        parse_map,
    ))
    .parse(input)
    .unwrap()
    .1
}
