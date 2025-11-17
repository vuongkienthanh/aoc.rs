use aoc_helper::nom::parse_number;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    sequence::preceded,
};

pub fn parse_input(input: &str) -> IResult<&str, (usize, usize, usize)> {
    (
        preceded(tag("Hit Points: "), parse_number),
        preceded(tag("Damage: "), parse_number),
        preceded(tag("Armor: "), parse_number),
    )
        .parse(input)
}
