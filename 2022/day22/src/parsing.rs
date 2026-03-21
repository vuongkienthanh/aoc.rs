use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};
use aoc_helper::direction::Direction;

#[derive(Debug, Clone, Default)]
pub enum Cell {
    #[default]
    Empty,
    Wall,
    Space,
}

#[derive(Debug)]
pub enum Op {
    Mv(usize),
    L,
    R,
}

fn parse_map_line(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(alt((
        tag(" ").map(|_| Cell::Empty),
        tag(".").map(|_| Cell::Space),
        tag("#").map(|_| Cell::Wall),
    )))
    .parse(input)
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    separated_list1(line_ending, parse_map_line).parse(input)
}

fn parse_ops(input: &str) -> IResult<&str, Vec<Op>> {
    many1(alt((
        complete::usize.map(Op::Mv),
        tag("L").map(|_| Op::L),
        tag("R").map(|_| Op::R),
    )))
    .parse(input)
}

pub fn parse_input(input: &str) -> (Vec<Vec<Cell>>, Vec<Op>) {
    all_consuming(separated_pair(
        parse_map,
        (line_ending, line_ending),
        parse_ops,
    ))
    .parse(input)
    .unwrap()
    .1
}