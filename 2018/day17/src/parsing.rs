use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

#[derive(Debug)]
pub enum Item {
    X(usize, (usize, usize)),
    Y(usize, (usize, usize)),
}

fn parse_x(input: &str) -> IResult<&str, Item> {
    (
        tag("x="),
        complete::usize,
        tag(", y="),
        complete::usize,
        tag(".."),
        complete::usize,
    )
        .map(|(_, a, _, b, _, c)| Item::X(a, (b, c)))
        .parse(input)
}
fn parse_y(input: &str) -> IResult<&str, Item> {
    (
        tag("y="),
        complete::usize,
        tag(", x="),
        complete::usize,
        tag(".."),
        complete::usize,
    )
        .map(|(_, a, _, b, _, c)| Item::Y(a, (b, c)))
        .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    alt((parse_x, parse_y)).parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
