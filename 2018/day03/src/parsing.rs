use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

pub type Item = (usize, usize, usize, usize, usize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        preceded(tag("#"), complete::usize),
        tag(" @ "),
        separated_pair(complete::usize, tag(","), complete::usize),
        tag(": "),
        separated_pair(complete::usize, tag("x"), complete::usize),
    )
        .map(|(idx, _, (col, row), _, (wide, tall))| (idx, row, col, wide, tall))
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
