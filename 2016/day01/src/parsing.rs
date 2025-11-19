use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
};
// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

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
        map_res(digit1, str::parse),
    )
        .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(tag(", "), parse_item).parse(input)
}
