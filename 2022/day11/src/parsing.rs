use crate::{Op, part1::Monkey};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded},
};

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    (
        delimited(tag("Monkey "), complete::usize, tag(":\n")),
        delimited(
            (space1, tag("Starting items: ")),
            separated_list1(tag(", "), complete::usize),
            line_ending,
        ),
        delimited(
            (space1, tag("Operation: new = ")),
            alt((
                preceded(tag("old * "), complete::usize).map(Op::Mul),
                preceded(tag("old + "), complete::usize).map(Op::Add),
                tag("old * old").map(|_| Op::Square),
            )),
            line_ending,
        ),
        delimited(
            (space1, tag("Test: divisible by ")),
            complete::usize,
            line_ending,
        ),
        delimited(
            (space1, tag("If true: throw to monkey ")),
            complete::usize,
            line_ending,
        ),
        preceded((space1, tag("If false: throw to monkey ")), complete::usize),
    )
        .map(|(_, items, op, div, iftrue, iffalse)| Monkey {
            items,
            op,
            div,
            iftrue,
            iffalse,
        })
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    all_consuming(separated_list1((line_ending, line_ending), parse_monkey))
        .parse(input)
        .unwrap()
        .1
}