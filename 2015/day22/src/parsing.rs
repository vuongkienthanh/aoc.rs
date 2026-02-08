use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    sequence::{preceded, separated_pair},
};

fn parse_hp(input: &str) -> IResult<&str, usize> {
    preceded(tag("Hit Points: "), complete::usize).parse(input)
}
fn parse_atk(input: &str) -> IResult<&str, usize> {
    preceded(tag("Damage: "), complete::usize).parse(input)
}

pub fn parse_input(input: &str) -> (usize, usize) {
    all_consuming(separated_pair(parse_hp, line_ending, parse_atk))
        .parse(input)
        .unwrap()
        .1
}
