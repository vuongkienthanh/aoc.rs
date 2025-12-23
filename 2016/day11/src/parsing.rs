use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Item<'a> {
    Generator(&'a str),
    Microchip(&'a str),
}

fn parse_generator<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    delimited(tag(" a "), alpha1.map(Item::Generator), tag(" generator")).parse(input)
}
fn parse_microchip<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    delimited(
        tag(" a "),
        alpha1.map(Item::Microchip),
        tag("-compatible microchip"),
    )
    .parse(input)
}
fn parse_item<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    alt((parse_generator, parse_microchip)).parse(input)
}
fn parse_items<'a>(input: &'a str) -> IResult<&'a str, Vec<Item<'a>>> {
    separated_list1(tag(","), parse_item).parse(input)
}
fn parse_last_item<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    delimited(alt((tag(", and"), tag(" and"))), parse_item, tag(".")).parse(input)
}

fn parse_all_items<'a>(input: &'a str) -> IResult<&'a str, Vec<Item<'a>>> {
    alt((
        tag(" nothing relevant.").map(|_| vec![]),
        (parse_items, parse_last_item).map(|(mut v, i)| {
            v.push(i);
            v
        }),
        terminated(parse_item.map(|x| vec![x]), tag(".")),
    ))
    .parse(input)
}

fn parse_floor_head(input: &str) -> IResult<&str, &str> {
    delimited(tag("The "), alpha1, tag(" floor contains")).parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Vec<Item<'a>>> {
    preceded(parse_floor_head, parse_all_items).parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<Vec<Item<'a>>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
