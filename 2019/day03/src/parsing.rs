use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};
use aoc_helper::direction::Direction;

type Item = (Direction, usize);
fn parse_item(input: &str) -> IResult<&str, Item> {
    (
        alt((
            complete::char('U').map(|_| Direction::Up),
            complete::char('D').map(|_| Direction::Down),
            complete::char('R').map(|_| Direction::Right),
            complete::char('L').map(|_| Direction::Left),
        )),
        complete::usize,
    )
        .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(tag(","), parse_item).parse(input)
}

pub fn parse_input(input: &str) -> (Vec<Item>, Vec<Item>) {
    all_consuming(separated_pair(parse_line, line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
