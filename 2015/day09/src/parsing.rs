use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};
use std::collections::HashMap;

pub type Mapping<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

fn parse_line(input: &str) -> IResult<&str, (&str, &str, usize)> {
    separated_pair(
        separated_pair(alpha1, tag(" to "), alpha1),
        tag(" = "),
        complete::usize,
    )
    .map(|((x, y), z)| (x, y, z))
    .parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Mapping<'a> {
    all_consuming(separated_list1(line_ending, parse_line).map(|v| {
        v.into_iter()
            .fold(Mapping::new(), |mut acc, (loc1, loc2, distance)| {
                acc.entry(loc1).or_default().insert(loc2, distance);
                acc.entry(loc2).or_default().insert(loc1, distance);
                acc
            })
    }))
    .parse(input)
    .unwrap()
    .1
}
