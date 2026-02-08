use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::{all_consuming, recognize},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

fn parse_name(input: &str) -> IResult<&str, &str> {
    recognize((alpha1, tag(" "), alpha1)).parse(input)
}
fn parse_bag_tag(input: &str) -> IResult<&str, &str> {
    alt((tag(" bags"), tag(" bag"))).parse(input)
}

fn parse_bag(input: &str) -> IResult<&str, (usize, &str)> {
    terminated(
        separated_pair(complete::usize, tag(" "), parse_name),
        parse_bag_tag,
    )
    .parse(input)
}
fn parse_bags(input: &str) -> IResult<&str, Vec<(usize, &str)>> {
    alt((
        tag("no other bags.").map(|_| vec![]),
        terminated(separated_list1(tag(", "), parse_bag), tag(".")),
    ))
    .parse(input)
}

pub type Item<'a> = (&'a str, Vec<(usize, &'a str)>);

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Item<'a>> {
    separated_pair(parse_name, tag(" bags contain "), parse_bags).parse(input)
}

pub fn parse_input<'a>(input: &'a str) -> Vec<Item<'a>> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
