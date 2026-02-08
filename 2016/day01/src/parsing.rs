use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char},
    combinator::all_consuming,
    multi::separated_list1,
};

type Item = (Turn, isize);

#[derive(Debug)]
pub enum Turn {
    Right,
    Left,
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    (
        alt((
            char('R').map(|_| Turn::Right),
            char('L').map(|_| Turn::Left),
        )),
        complete::isize,
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(tag(", "), parse_item))
        .parse(input)
        .unwrap()
        .1
}
