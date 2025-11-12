pub mod part1;
pub mod part2;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
};

type Coord = (usize, usize);
#[derive(Debug, PartialEq)]
enum Action {
    On,
    Off,
    Toggle,
}
fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}
fn parse_coord(input: &str) -> IResult<&str, Coord> {
    separated_pair(parse_number, tag(","), parse_number).parse(input)
}
fn parse_line(input: &str) -> IResult<&str, (Action, Coord, Coord)> {
    let (input, action) = alt((
        tag("turn on ").map(|_| Action::On),
        tag("turn off ").map(|_| Action::Off),
        tag("toggle ").map(|_| Action::Toggle),
    ))
    .parse(input)?;
    let (input, (top_left, bottom_right)) =
        separated_pair(parse_coord, tag(" through "), parse_coord).parse(input)?;
    Ok((input, (action, top_left, bottom_right)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Action, Coord, Coord)>> {
    separated_list1(line_ending, parse_line).parse(input)
}
