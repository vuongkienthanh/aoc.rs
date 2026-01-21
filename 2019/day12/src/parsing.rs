use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
};

type Item = (isize, isize, isize);

fn parse_line(input: &str) -> IResult<&str, Item> {
    (
        tag("<x="),
        complete::isize,
        tag(", y="),
        complete::isize,
        tag(", z="),
        complete::isize,
        tag(">"),
    )
        .map(|(_, a, _, b, _, c, _)| (a, b, c))
        .parse(input)
}

pub fn parse_input(input: &str) -> Vec<Item> {
    all_consuming(separated_list1(line_ending, parse_line))
        .parse(input)
        .unwrap()
        .1
}
