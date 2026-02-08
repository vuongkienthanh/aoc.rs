pub mod part1;
pub mod part2;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::cmp::Ordering;
use std::collections::HashMap;

type Rule<'a> = (&'a str, &'a str);
type Rules<'a> = HashMap<&'a str, HashMap<&'a str, Ordering>>;
type Update<'a> = Vec<&'a str>;
type Updates<'a> = Vec<Update<'a>>;

fn rule(input: &str) -> IResult<&str, Rule> {
    separated_pair(digit1, tag("|"), digit1)(input)
}
fn rules(input: &str) -> IResult<&str, Rules> {
    fold_many1(
        terminated(rule, line_ending),
        Rules::new,
        |mut acc, (a, b)| {
            acc.entry(a).or_default().insert(b, Ordering::Less);
            acc.entry(b).or_default().insert(a, Ordering::Greater);
            acc
        },
    )(input)
}
fn update(input: &str) -> IResult<&str, Update> {
    separated_list1(tag(","), digit1)(input)
}
fn updates(input: &str) -> IResult<&str, Updates> {
    separated_list1(line_ending, update)(input)
}

pub fn parse(input: &str) -> IResult<&str, (Rules, Updates)> {
    separated_pair(rules, line_ending, updates)(input)
}
