use crate::{
    Op,
    part1::Monkey1,
    part2::{Item, Monkey2},
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded},
};

fn parse_item(input: &str) -> IResult<&str, (Vec<usize>, Op, usize, usize, usize)> {
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
        .map(|(_, items, op, div, iftrue, iffalse)| (items, op, div, iftrue, iffalse))
        .parse(input)
}

fn parse_monkey1(input: &str) -> IResult<&str, Monkey1> {
    parse_item
        .map(|(items, op, div, iftrue, iffalse)| Monkey1 {
            items,
            op,
            div,
            iftrue,
            iffalse,
        })
        .parse(input)
}

fn parse_monkey2(input: &str) -> IResult<&str, Monkey2> {
    parse_item
        .map(|(items, op, div, iftrue, iffalse)| Monkey2 {
            items: items
                .into_iter()
                .map(|i| Item {
                    div2: i % 2,
                    div3: i % 3,
                    div5: i % 5,
                    div7: i % 7,
                    div11: i % 11,
                    div13: i % 13,
                    div17: i % 17,
                    div19: i % 19,
                })
                .collect(),
            op,
            div,
            iftrue,
            iffalse,
        })
        .parse(input)
}

pub fn parse_input1(input: &str) -> Vec<Monkey1> {
    all_consuming(separated_list1((line_ending, line_ending), parse_monkey1))
        .parse(input)
        .unwrap()
        .1
}

pub fn parse_input2(input: &str) -> Vec<Monkey2> {
    all_consuming(separated_list1((line_ending, line_ending), parse_monkey2))
        .parse(input)
        .unwrap()
        .1
}
