use fxhash::FxHashMap;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, anychar, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

pub type Rules = FxHashMap<usize, Rule>;

#[derive(Debug, Clone)]
pub enum Rule {
    Char(char),
    Or(Vec<Vec<usize>>),
}

fn parse_rule_line(input: &str) -> IResult<&str, (usize, Rule)> {
    separated_pair(complete::usize, tag(": "), parse_rule).parse(input)
}
fn parse_char(input: &str) -> IResult<&str, Rule> {
    delimited(tag("\""), anychar, tag("\""))
        .map(|x| Rule::Char(x))
        .parse(input)
}

fn parse_or(input: &str) -> IResult<&str, Rule> {
    separated_list1(tag(" | "), separated_list1(tag(" "), complete::usize))
        .map(|v| Rule::Or(v))
        .parse(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_char, parse_or)).parse(input)
}
fn parse_rules(input: &str) -> IResult<&str, FxHashMap<usize, Rule>> {
    separated_list1(line_ending, parse_rule_line)
        .map(|v: Vec<(usize, Rule)>| {
            v.into_iter().fold(FxHashMap::default(), |mut acc, (i, r)| {
                acc.insert(i, r);
                acc
            })
        })
        .parse(input)
}

fn parse_msg(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        (line_ending, line_ending),
        separated_list1(line_ending, alpha1),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> (Rules, Vec<&str>) {
    all_consuming((parse_rules, parse_msg))
        .parse(input)
        .unwrap()
        .1
}
