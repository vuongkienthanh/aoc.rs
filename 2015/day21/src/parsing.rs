use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    sequence::preceded,
};
fn parse_hp(input: &str) -> IResult<&str, usize> {
    preceded(tag("Hit Points: "), complete::usize).parse(input)
}
fn parse_atk(input: &str) -> IResult<&str, usize> {
    preceded(tag("Damage: "), complete::usize).parse(input)
}
fn parse_def(input: &str) -> IResult<&str, usize> {
    preceded(tag("Armor: "), complete::usize).parse(input)
}

pub fn parse_input(input: &str) -> (usize, usize, usize) {
    all_consuming(
        (parse_hp, line_ending, parse_atk, line_ending, parse_def).map(|(a, _, b, _, c)| (a, b, c)),
    )
    .parse(input)
    .unwrap()
    .1
}
