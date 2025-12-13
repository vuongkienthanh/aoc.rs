use aoc_helper::nom::parse_number;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::line_ending,
    sequence::{preceded, terminated},
};

pub fn parse_input(input: &str) -> IResult<&str, (usize, usize, usize)> {
    (
        terminated(preceded(tag("Hit Points: "), parse_number), line_ending),
        terminated(preceded(tag("Damage: "), parse_number), line_ending),
        preceded(tag("Armor: "), parse_number),
    )
        .parse(input)
}
