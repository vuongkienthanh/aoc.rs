use aoc_helper::nom::line;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded},
};

// x, y, used, avail
pub type Item = (usize, usize, usize, usize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        preceded(tag("/dev/grid/node-x"), complete::usize),
        preceded(tag("-y"), complete::usize),
        delimited(multispace1, complete::usize, tag("T")),
        delimited(multispace1, complete::usize, tag("T")),
        delimited(multispace1, complete::usize, tag("T")),
        delimited(multispace1, complete::usize, tag("%")),
    )
        .map(|(x, y, _, u, v, _)| (x, y, u, v))
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    let (input, _) = line(input).unwrap();
    let (input, _) = line(input).unwrap();
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
