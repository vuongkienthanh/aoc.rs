use aoc_helper::nom::{line, parse_number};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
};

// x, y, used, avail
pub type Item = (usize, usize, usize, usize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        preceded(tag("/dev/grid/node-x"), parse_number),
        preceded(tag("-y"), parse_number),
        delimited(multispace1, parse_number, tag("T")),
        delimited(multispace1, parse_number, tag("T")),
        delimited(multispace1, parse_number, tag("T")),
        delimited(multispace1, parse_number, tag("%")),
    )
        .map(|(x, y, _, u, v, _)| (x, y, u, v))
        .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Item>> {
    let (input, _) = line(input)?;
    let (input, _) = line(input)?;
    separated_list1(line_ending, parse_line).parse(input)
}
