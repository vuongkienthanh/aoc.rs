#[allow(unused_imports)]
// use aoc_helper::nom::parse_signed_usize;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

// month, day, hour, min
pub type Time = (usize, usize, usize, usize);

#[derive(Debug)]
pub enum Item {
    Shift(usize),
    Wake,
    Sleep,
}

fn parse_time(input: &str) -> IResult<&str, Time> {
    (
        tag("[1518-"),
        complete::usize,
        tag("-"),
        complete::usize,
        tag(" "),
        complete::usize,
        tag(":"),
        complete::usize,
        tag("] "),
    )
        .map(|(_, m, _, d, _, h, _, mn, _)| (m, d, h, mn))
        .parse(input)
}
fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        delimited(tag("Guard #"), complete::usize, tag(" begins shift")).map(Item::Shift),
        tag("wakes up").map(|_| Item::Wake),
        tag("falls asleep").map(|_| Item::Sleep),
    ))
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (Time, Item)> {
    (parse_time, parse_item).parse(input)
}

pub fn parse_input(input: &str) -> Vec<(Time, Item)> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
