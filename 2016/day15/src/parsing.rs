use aoc_helper::algebra::chinese_remainder_theorem::Item;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        tag("Disc #"),
        complete::usize,
        tag(" has "),
        complete::usize,
        tag(" positions; at time=0, it is at position "),
        complete::usize,
        tag("."),
    )
        .map(|(_, a, _, b, _, c, _)| Item {
            modulus: b,
            rem: b - ((a + c) % b),
        })
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
