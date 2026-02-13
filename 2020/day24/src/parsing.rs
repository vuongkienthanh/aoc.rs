use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::all_consuming,
    multi::{many1, separated_list1},
};

pub enum Item {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

fn parse_line(input: &str) -> IResult<&str, Vec<Item>> {
    many1(alt((
        tag("e").map(|_| Item::E),
        tag("w").map(|_| Item::W),
        tag("ne").map(|_| Item::NE),
        tag("nw").map(|_| Item::NW),
        tag("se").map(|_| Item::SE),
        tag("sw").map(|_| Item::SW),
    )))
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<Item>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
